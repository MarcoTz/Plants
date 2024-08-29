pub mod bot;
pub mod bot_actions;
pub mod commands;
pub mod config;
pub mod errors;
pub mod fsm;

use bot_api::{
    bot::Bot,
    commands::Command,
    handlers::{CommandHandler, ErrorHandler, MessageHandler},
    message::Message,
    run_bot,
};
use config::load_config;
use errors::Error;
use log::Level;
use logger::{file_logger::FileLogger, init::init_logger};
use std::process::exit;

static LOGGER: FileLogger = FileLogger {
    level: Level::Info,
    file_path: "log.txt",
};

#[derive(Clone, Debug)]
struct Handler;

impl Command for Handler {
    type Error = Error;
    fn from_str(_: &str) -> Result<Handler, Error> {
        Ok(Handler {})
    }
}

impl MessageHandler for Handler {
    type Error = Error;
    fn handle(&self, _: &Bot, msg: Message) -> Result<(), Error> {
        println!("{msg:?}");
        Ok(())
    }
}
impl CommandHandler<Handler> for Handler {
    type Error = Error;
    fn handle(&self, _: &Bot, cmd: Handler) -> Result<(), Error> {
        println!("{cmd:?}");
        Ok(())
    }
}
impl<'a> ErrorHandler<'a> for Handler {
    fn handle_error(&self, err: Box<dyn std::error::Error + 'a>) {
        println!("{err:?}")
    }
}

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
    run_bot(&mut bot, &Handler {}, &Handler {}, &Handler {}).await;
}
