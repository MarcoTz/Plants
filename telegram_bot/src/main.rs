pub mod action_handler;
pub mod bot_actions;
pub mod commands;
pub mod config;
pub mod errors;

use action_handler::ActionHandler;
use bot_api::{bot::Bot, run_bot};
use config::load_config;
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
    log::info!("Succesfully set up logger");

    let conf = match load_config() {
        Ok(conf) => conf,
        Err(err) => {
            println!("{err:?}");
            exit(1)
        }
    };
    log::info!("Successfully loaded config");

    let mut bot = Bot::new(conf.api_key);
    let mut handler = ActionHandler::default();
    log::info!("Running bot");
    if let Err(err) = run_bot(&mut bot, &mut handler).await {
        log::info!("Bot exited with error");
        println!("{err}");
    }
}
