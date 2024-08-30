use super::{
    bot_actions::{Action, BotAction},
    commands::{Command, CommandRes},
    errors::Error,
};
use bot_api::{bot::Bot, handlers::Handler, message::Message};
use database::{database_manager::DatabaseManager, file_backend::FileDB};

pub enum ImmediateAction {
    Push,
    CheckLogs,
}
pub struct ActionHandler<T: DatabaseManager> {
    pub current_action: BotAction,
    pub db_man: T,
}

impl Default for ActionHandler<FileDB> {
    fn default() -> ActionHandler<FileDB> {
        ActionHandler {
            current_action: BotAction::Idle,
            db_man: FileDB::default(),
        }
    }
}

impl<T: DatabaseManager> ActionHandler<T> {
    fn check_action(&mut self) -> Result<Option<String>, Error> {
        if self.current_action.is_done() {
            let ret_msg = self.current_action.write_result(&mut self.db_man)?;
            self.current_action = BotAction::Idle;
            Ok(Some(ret_msg))
        } else {
            Ok(None)
        }
    }

    pub fn handle_message(&mut self, message: Message) -> String {
        match self.handle_input(message.text) {
            Ok(ret_msg) => ret_msg,
            Err(err) => format!("{err}"),
        }
    }

    pub fn handle_input(&mut self, new_input: Option<String>) -> Result<String, Error> {
        let input = new_input.ok_or(Error::MissingInput("Message".to_owned()))?;
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
            ImmediateAction::Push => Ok("Not yet implemented".to_owned()),
            ImmediateAction::CheckLogs => Ok("Not yet implemented".to_owned()),
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
            CommandRes::NewInput(inp) => self.handle_input(Some(inp)),
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
        let ret_msg = self.process_command(cmd);
        bot.send_message(message.chat.id.to_string(), ret_msg)
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, bot: &Bot, msg: Message) -> Result<(), Error> {
        log::info!("Handling message");
        let chat_id = msg.chat.id.to_string();
        let ret_msg = self.handle_message(msg);
        bot.send_message(chat_id, ret_msg).await?;
        Ok(())
    }
}
