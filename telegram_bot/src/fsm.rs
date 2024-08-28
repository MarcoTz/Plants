use super::{
    bot_actions::{Action, BotAction},
    errors::Error,
};
use database::{database_manager::DatabaseManager, file_backend::FileDB};

pub struct BotFSM<T: DatabaseManager> {
    pub current_action: BotAction,
    pub db_man: T,
}

impl Default for BotFSM<FileDB> {
    fn default() -> BotFSM<FileDB> {
        BotFSM {
            current_action: BotAction::Idle,
            db_man: FileDB::default(),
        }
    }
}

impl<T: DatabaseManager> BotFSM<T> {
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
}
