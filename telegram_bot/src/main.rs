pub mod bot;
pub mod bot_actions;
pub mod commands;
pub mod config;
pub mod errors;
pub mod fsm;

use bot::schema;
use config::load_config;
use errors::Error;
use fsm::BotFSM;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conf = load_config()?;
    let fsm = BotFSM::default();
    Ok(())
}
