use super::{
    bot_actions::{Action, BotAction},
    errors::Error,
};
use database::database_manager::DatabaseManager;

pub struct BotFSM<T: DatabaseManager> {
    pub current_action: BotAction,
    pub db_man: T,
}

impl<T: DatabaseManager> BotFSM<T> {
    pub fn handle_input(&mut self, new_input: String) -> Result<String, Error> {
        self.current_action
            .handle_input(new_input, &mut self.db_man)?;
        let ret_msg;
        if self.current_action.is_done() {
            ret_msg = self.current_action.write_result(&mut self.db_man)?;
            self.current_action = BotAction::Idle;
        } else {
            ret_msg = self.current_action.get_next_prompt()?;
        }
        Ok(ret_msg)
    }
}
