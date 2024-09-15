use super::{
    bot_actions::{Action, BotAction},
    commands::{Command, CommandRes},
    errors::{CommandError, Error},
};
use bot_api::{bot::Bot, handlers::Handler, message::Message};
use bytes::Bytes;
use chrono::Local;
use database::{database_manager::DatabaseManager, file_backend::FileDB};
use std::{fs::File, io::Write, path::PathBuf, process, process::exit, str};

#[derive(Debug, PartialEq, Eq)]
pub enum ImmediateAction {
    Push,
    CheckLogs,
    Exit,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActionHandler<T: DatabaseManager> {
    pub current_action: BotAction,
    pub white_list: Vec<i64>,
    pub plants_dir: PathBuf,
    pub db_man: T,
}

impl Default for ActionHandler<FileDB> {
    fn default() -> ActionHandler<FileDB> {
        ActionHandler {
            current_action: BotAction::Idle,
            white_list: vec![],
            plants_dir: PathBuf::from("data/Plants"),
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

    pub fn save_image(&self, img: Bytes, msg: Message) -> Result<PathBuf, Error> {
        let plant_name = msg
            .caption
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

            let ret_msg = self.current_action.get_next_prompt()?;
            Ok(ret_msg)
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
            ImmediateAction::CheckLogs => Ok("Not yet implemented".to_owned()),
            ImmediateAction::Exit => exit(0),
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
}

impl<T: DatabaseManager> Handler<Command> for ActionHandler<T> {
    type Error = Error;
    async fn handle_command(
        &mut self,
        bot: &Bot,
        cmd: Command,
        message: Message,
    ) -> Result<(), Self::Error> {
        log::info!("Handling Command {cmd}");
        let user = message.from.ok_or(Error::Unauthorized("".to_owned()))?;
        if self.white_list.contains(&user.id) {
            let ret_msg = self.process_command(cmd);
            bot.send_message(message.chat.id.to_string(), ret_msg)
                .await?;
            Ok(())
        } else {
            bot.send_message(
                message.chat.id.to_string(),
                "User is not authorized".to_owned(),
            )
            .await?;

            Ok(())
        }
    }

    async fn handle_message(&mut self, bot: &Bot, msg: Message) -> Result<(), Error> {
        log::info!("Handling message");
        let user = msg.from.clone().ok_or(Error::Unauthorized("".to_owned()))?;
        if self.white_list.contains(&user.id) {
            let chat_id = msg.chat.id.to_string();
            if let Some(photos) = msg.photo.clone() {
                let img_biggest = photos.get_biggest()?;
                let res = bot.download_image(img_biggest.file_id.clone()).await?;
                match self.save_image(res, msg) {
                    Ok(out_path) => {
                        bot.send_message(chat_id, format!("Successfully saved image {out_path:?}"))
                            .await?
                    }
                    Err(err) => bot.send_message(chat_id, format!("{err}")).await?,
                }
                Ok(())
            } else if let Some(text) = msg.text {
                match self.handle_input(text) {
                    Ok(ret_msg) => {
                        bot.send_message(chat_id, ret_msg).await?;
                        Ok(())
                    }
                    Err(err) => {
                        bot.send_message(chat_id, format!("{err}")).await?;
                        Ok(())
                    }
                }
            } else {
                bot.send_message(chat_id, "Could not handle message".to_owned())
                    .await?;
                Ok(())
            }
        } else {
            bot.send_message(msg.chat.id.to_string(), "User is unauthorized".to_owned())
                .await?;
            Ok(())
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
            plants_dir: PathBuf::from("data/plants"),
            db_man: DummyManager {},
        }
    }

    #[test]
    fn handler_default() {
        let result = ActionHandler::default();
        let expected = ActionHandler {
            current_action: BotAction::Idle,
            white_list: vec![],
            plants_dir: PathBuf::from("data/plants"),
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
        let expected = "Possible commands: \n\n help -- Display Help Text\nwater -- Water plants (today)\nwater_location -- Water all plants in location (today)\nfertilize -- Fertilize plants (today)\nrain -- It rained (all outside plants will be watered)\nnew_growth -- Enter new growth\nnew_plant -- Enter new plant\nnew_species -- Enter new species\nnew_activity -- Enter new activity\nupdate_species -- Update species\nupdate_plant -- Update plant\ntoday -- Enter the current date as input\nmove_to_graveyard -- Move Plant to graveyard\nabort -- Abort the current action\npush -- Push local changes to github\ncheck_logs -- Check warnings generated from build\nexit -- Exit the bot";
        assert_eq!(result, expected)
    }

    #[test]
    fn handle_cmd_action() {
        let result = example_handler().process_command(Command::Abort);
        let expected = "Currently there is no active action, please try again";
        assert_eq!(result, expected)
    }

    #[test]
    fn handle_cmd_input() {
        let result = example_handler().process_command(Command::Today);
        let expected = "Currently there is no active action, please try again";
        assert_eq!(result, expected)
    }
}
