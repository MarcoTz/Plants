use super::{
    bot_actions::{Action, BotAction},
    commands::{Command, CommandRes},
    errors::Error,
};
use bot_api::{
    bot::Bot,
    handlers::{CommandHandler, MessageHandler},
    message::Message,
};
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
    pub fn handle_input(&mut self, new_input: Option<&str>) -> Result<String, Error> {
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

    pub fn new_action(&mut self, new_action: BotAction) -> Result<String, Error> {
        if self.current_action == BotAction::Idle {
            self.current_action = new_action;
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

    fn process_command(&mut self, cmd: Command) -> Result<String, Error> {
        match cmd.get_res() {
            CommandRes::Message(msg) => Ok(msg),
            CommandRes::NewAction(action) => self.new_action(action),
            CommandRes::NewInput(inp) => self.handle_input(Some(&inp)),
            CommandRes::ImmediateAction(act) => self.handle_immediate(&act),
        }
    }
}

impl<T: DatabaseManager> CommandHandler<Command> for ActionHandler<T> {
    type Error = Error;
    async fn handle(
        &mut self,
        bot: &Bot,
        cmd: Command,
        message: Message,
    ) -> Result<(), Self::Error> {
        let ret_msg = self.process_command(cmd)?;
        let send_res = bot
            .send_message(message.chat.id.to_string(), ret_msg)
            .await?;
        Ok(())
    }
}

impl<T: DatabaseManager> MessageHandler for ActionHandler<T> {
    type Error = Error;
    async fn handle(&mut self, bot: &Bot, msg: Message) -> Result<(), Self::Error> {
        let msg_text = msg.get_text()?;
        let ret_msg = self.handle_input(Some(&msg_text))?;
        bot.send_message(msg.chat.id.to_string(), ret_msg).await?;
        Ok(())
    }
}
