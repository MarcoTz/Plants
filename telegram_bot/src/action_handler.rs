use super::{
    bot_actions::{Action, BotAction},
    commands::{Command, CommandRes},
    errors::{CommandError, Error},
};
use bot_api::{bot::Bot, handlers::Handler, message::Message, photo_size::Photo};
use bytes::Bytes;
use chrono::Local;
use database::{database_manager::DatabaseManager, file_backend::FileDB};
use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    process, str,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ImmediateAction {
    Push,
    Abort,
    CheckLogs,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActionHandler<T: DatabaseManager> {
    pub current_action: BotAction,
    pub white_list: Vec<i64>,
    pub plants_dir: PathBuf,
    pub log_path: PathBuf,
    pub db_man: T,
}

impl Default for ActionHandler<FileDB> {
    fn default() -> ActionHandler<FileDB> {
        ActionHandler {
            current_action: BotAction::Idle,
            white_list: vec![],
            plants_dir: PathBuf::from("data/Plants"),
            log_path: PathBuf::from("./build.log"),
            db_man: FileDB::default(),
        }
    }
}

impl<T: DatabaseManager> ActionHandler<T> {
    pub fn new(white_list: Vec<i64>, db_man: T) -> Self {
        ActionHandler {
            current_action: BotAction::Idle,
            white_list,
            plants_dir: PathBuf::from("data/Plants"),
            log_path: PathBuf::from("./build.log"),
            db_man,
        }
    }
    fn check_action(&mut self) -> Result<Option<String>, Error> {
        if self.current_action.is_done() {
            let ret_msg = self.current_action.write_result(&mut self.db_man)?;
            self.current_action = BotAction::Idle;
            Ok(Some(ret_msg))
        } else {
            Ok(None)
        }
    }

    pub fn save_image(&self, img: Bytes, msg: &Message) -> Result<PathBuf, Error> {
        let plant_name = msg
            .caption
            .clone()
            .ok_or(Error::MissingInput("Plant Name".to_owned()))?;
        let img_name = Local::now().date_naive().format("%d%m%Y.jpg").to_string();
        let plant_path = self.plants_dir.join(plant_name.replace(' ', ""));
        println!("{plant_path:?}");
        let out_path = plant_path.join(img_name);
        let mut out_file =
            File::create(out_path.clone()).map_err(|err| Error::Other(Box::new(err)))?;
        out_file
            .write_all(&img)
            .map_err(|err| Error::Other(Box::new(err)))?;
        out_file
            .flush()
            .map_err(|err| Error::Other(Box::new(err)))?;
        Ok(out_path)
    }

    pub fn handle_input(&mut self, input: String) -> Result<String, Error> {
        if let Some(ret_msg) = self.check_action()? {
            Ok(ret_msg)
        } else {
            self.current_action
                .handle_input(input.to_owned(), &mut self.db_man)?;

            if let Some(ret_msg) = self.check_action()? {
                Ok(ret_msg)
            } else {
                let ret_msg = self.current_action.get_next_prompt()?;
                Ok(ret_msg)
            }
        }
    }

    pub fn handle_immediate(&mut self, action: &ImmediateAction) -> Result<String, Error> {
        match action {
            ImmediateAction::Push => {
                let output_add = process::Command::new("git")
                    .arg("add")
                    .arg("-A")
                    .output()
                    .map_err(|err| CommandError {
                        cmd: "git add -A".to_owned(),
                        msg: err.to_string(),
                    })?;
                if output_add.status.success() {
                    Ok(())
                } else {
                    Err(CommandError {
                        cmd: "git add -A".to_owned(),
                        msg: str::from_utf8(&output_add.stderr)
                            .unwrap_or("Could not stage changes")
                            .to_owned(),
                    })
                }?;

                let output_commit = process::Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg(format!(
                        "autocommit_{}",
                        Local::now().date_naive().format("%d%m%Y")
                    ))
                    .output()
                    .map_err(|err| CommandError {
                        cmd: "git commit -m".to_owned(),
                        msg: err.to_string(),
                    })?;

                if output_commit.status.success() {
                    Ok(())
                } else {
                    Err(CommandError {
                        cmd: "git commit -m".to_owned(),
                        msg: str::from_utf8(&output_commit.stderr)
                            .unwrap_or("Could not commit changes")
                            .to_owned(),
                    })
                }?;

                let output_push =
                    process::Command::new("git")
                        .arg("push")
                        .output()
                        .map_err(|err| CommandError {
                            cmd: "git push".to_owned(),
                            msg: err.to_string(),
                        })?;
                if output_push.status.success() {
                    Ok(())
                } else {
                    Err(CommandError {
                        cmd: "git push".to_owned(),
                        msg: str::from_utf8(&output_push.stderr)
                            .unwrap_or("Could not push changes")
                            .to_owned(),
                    })
                }?;
                Ok("Successfully pushed changes".to_owned())
            }
            ImmediateAction::CheckLogs => {
                log::info!("Running static build for new log file");
                process::Command::new("./target/release/plant_website_static")
                    .output()
                    .map_err(|err| CommandError {
                        cmd: "render website".to_owned(),
                        msg: err.to_string(),
                    })?;
                log::info!("Loading Log File");

                let mut file =
                    File::open(self.log_path.clone()).map_err(|err| Error::Other(Box::new(err)))?;
                let mut contents: String = "".to_owned();
                file.read_to_string(&mut contents)
                    .map_err(|err| Error::Other(Box::new(err)))?;

                let mut lines = contents
                    .split("\n")
                    .filter(|line| line.contains("ERR") || line.contains("WARN"))
                    .filter_map(|line| line.split_once(" - ").map(|(_, s)| s))
                    .map(|line| line.to_owned())
                    .collect::<HashSet<String>>()
                    .into_iter()
                    .collect::<Vec<String>>();
                lines.sort();
                log::info!("Got {} logs", lines.len());
                if lines.is_empty() {
                    Ok("No errors or warnings".to_owned())
                } else {
                    Ok(lines.join("\n"))
                }
            }
            ImmediateAction::Abort => {
                let action = self.current_action.to_string();
                self.current_action = BotAction::Idle;
                Ok(format!("Aborted action {action}"))
            }
        }
    }

    pub fn new_action(&mut self, new_action: &BotAction) -> Result<String, Error> {
        if self.current_action == BotAction::Idle {
            self.current_action = new_action.clone();
            if let Some(ret_msg) = self.check_action()? {
                Ok(ret_msg)
            } else {
                let ret_msg = self.current_action.get_next_prompt()?;
                Ok(ret_msg)
            }
        } else {
            Err(Error::ActionAlreadyRunning(self.current_action.to_string()))
        }
    }

    fn process_command(&mut self, cmd: Command) -> String {
        let action_res = match cmd.get_res() {
            CommandRes::Message(msg) => Ok(msg),
            CommandRes::NewAction(action) => self.new_action(&action),
            CommandRes::NewInput(inp) => self.handle_input(inp),
            CommandRes::ImmediateAction(act) => self.handle_immediate(&act),
        };
        match action_res {
            Ok(res_msg) => res_msg,
            Err(err) => format!("{err}"),
        }
    }

    async fn authorize(&mut self, b: &Bot, msg: &Message) -> bool {
        let user = &msg.from;
        match user {
            None => {
                let _ = b
                    .send_message(msg.chat.id.to_string(), "User is not authorized".to_owned())
                    .await;
                false
            }
            Some(user) => self.white_list.contains(&user.id),
        }
    }

    async fn get_image(&mut self, b: &mut Bot, photo: Photo, msg: &Message) -> Result<(), Error> {
        let img_biggest = photo.get_biggest()?;
        let res = b.download_image(img_biggest.file_id.clone()).await?;
        match self.save_image(res, msg) {
            Ok(out_path) => {
                b.send_message(
                    msg.chat.id.to_string(),
                    format!("Successfully saved image {out_path:?}"),
                )
                .await?
            }
            Err(err) => {
                b.send_message(msg.chat.id.to_string(), format!("{err}"))
                    .await?
            }
        }
        Ok(())
    }
}

impl<T: DatabaseManager> Handler<Command> for ActionHandler<T> {
    async fn handle_msg(&mut self, b: &mut Bot, msg: Message) {
        if !(self.authorize(b, &msg).await) {
            return;
        };
        let text = msg.text.unwrap_or_default();
        if text.is_empty() {
            return;
        };
        match self.handle_input(text) {
            Ok(ret_msg) => {
                let _ = b.send_message(msg.chat.id.to_string(), ret_msg).await;
            }
            Err(err) => {
                let _ = b
                    .send_message(msg.chat.id.to_string(), format!("{err}"))
                    .await;
            }
        }
    }

    async fn handle_cmd(&mut self, b: &mut Bot, cmd: Command, msg: Message) {
        if !(self.authorize(b, &msg).await) {
            return;
        };
        let ret_msg = self.process_command(cmd);
        let _ = b.send_message(msg.chat.id.to_string(), ret_msg).await;
    }

    async fn handle_img(&mut self, b: &mut Bot, photo: Photo, msg: Message) {
        if !(self.authorize(b, &msg).await) {
            return;
        };
        match self.get_image(b, photo, &msg).await {
            Ok(()) => (),
            Err(err) => {
                let _ = b
                    .send_message(
                        msg.chat.id.to_string(),
                        format!("Could not save image: {err}"),
                    )
                    .await;
            }
        }
    }
}

#[cfg(test)]
mod action_handler_tests {
    use super::{ActionHandler, BotAction, Command};
    use crate::bot_actions::{NewPlant, Rain};
    use crate::test_common::DummyManager;
    use database::file_backend::FileDB;
    use std::path::PathBuf;

    fn example_handler() -> ActionHandler<DummyManager> {
        ActionHandler {
            current_action: BotAction::Idle,
            white_list: vec![],
            log_path: PathBuf::from("log.txt"),
            plants_dir: PathBuf::from("data/Plants"),
            db_man: DummyManager {},
        }
    }

    #[test]
    fn handler_default() {
        let result = ActionHandler::default();
        let expected = ActionHandler {
            current_action: BotAction::Idle,
            white_list: vec![],
            plants_dir: PathBuf::from("data/Plants"),
            log_path: PathBuf::from("log.txt"),
            db_man: FileDB::default(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_err() {
        let result = example_handler().check_action();
        assert!(result.is_err());
    }

    #[test]
    fn check_done() {
        let mut handler = example_handler();
        handler.current_action = BotAction::Rain(Rain {});
        let result = handler.check_action().unwrap();
        let expected = Some("Successfully watered plants: ".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn check_not_done() {
        let mut handler = example_handler();
        handler.current_action = BotAction::NewPlant(NewPlant::default());
        let result = handler.check_action().unwrap();
        assert_eq!(result, None)
    }

    #[test]
    fn handle_inp() {
        let mut handler = example_handler();
        handler.current_action = BotAction::NewPlant(NewPlant::default());
        let result = handler.handle_input("name".to_owned()).unwrap();
        let expected = "Please enter species";
        assert_eq!(result, expected)
    }
    #[test]
    fn handle_inp_err() {
        let result = example_handler().handle_input("".to_owned());
        assert!(result.is_err())
    }

    #[test]
    fn new_action() {
        let mut handler = example_handler();
        let result = handler.new_action(&BotAction::Rain(Rain {})).unwrap();
        let expected = "Successfully watered plants: ";
        assert_eq!(result, expected)
    }

    #[test]
    fn new_action_fail() {
        let mut handler = example_handler();
        handler.current_action = BotAction::NewPlant(NewPlant::default());
        let result = handler.new_action(&BotAction::Rain(Rain {}));
        assert!(result.is_err())
    }

    #[test]
    fn handle_cmd_msg() {
        let result = example_handler().process_command(Command::Help);
        let expected = "Possible commands:\n\n/help -- Display Help Text\n/water -- Water plants (today)\n/water_location -- Water all plants in location (today)\n/fertilize -- Fertilize plants (today)\n/rain -- It rained (all outside plants will be watered)\n/new_growth -- Enter new growth\n/new_plant -- Enter new plant\n/new_species -- Enter new species\n/new_activity -- Enter new activity\n/update_species -- Update species\n/update_plant -- Update plant\n/today -- Enter the current date as input\n/move_to_graveyard -- Move Plant to graveyard\n/abort -- Abort the current action\n/push -- Push local changes to github\n/check_logs -- Check warnings generated from build";
        assert_eq!(result, expected)
    }

    #[test]
    fn handle_cmd_action() {
        let result = example_handler().process_command(Command::Abort);
        let expected = "Aborted action Idle";
        assert_eq!(result, expected)
    }

    #[test]
    fn handle_cmd_input() {
        let result = example_handler().process_command(Command::Today);
        let expected = "Currently there is no active action, please try again";
        assert_eq!(result, expected)
    }
}
