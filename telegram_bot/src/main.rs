pub mod action_handler;
pub mod bot_actions;
pub mod commands;
pub mod config;
pub mod error_handler;
pub mod errors;

use action_handler::ActionHandler;
use bot_api::{bot::Bot, run_bot};
use config::load_config;
use error_handler::ErrHandler;
use log::Level;
use logger::{file_logger::FileLogger, init::init_logger};
use std::process::exit;

static LOGGER: FileLogger = FileLogger {
    level: Level::Info,
    file_path: "log.txt",
};

#[tokio::main]
async fn main() {
    let log_res = init_logger(&LOGGER);
    if log_res.is_err() {
        println!("{}", log_res.unwrap_err());
        exit(1);
    }
    let conf = match load_config() {
        Ok(conf) => conf,
        Err(err) => {
            println!("{err:?}");
            exit(1)
        }
    };

    let mut bot = Bot::new(conf.api_key);
    let mut handler = ActionHandler::default();
    let err_handler = ErrHandler {};
    run_bot(&mut bot, &mut handler, &err_handler).await;
}
