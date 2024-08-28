pub mod bot;
pub mod bot_actions;
pub mod commands;
pub mod config;
pub mod errors;
pub mod fsm;

use bot_api::bot::Bot;
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
    let conf = match load_config() {
        Ok(conf) => conf,
        Err(err) => {
            println!("{err:?}");
            exit(1)
        }
    };

    let bot = Bot {
        api_key: conf.api_key,
    };

    match bot.get_all_updates().await {
        Ok(_) => println!("Success"),
        Err(err) => println!("{err:?}"),
    }
}
