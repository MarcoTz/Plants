pub mod bot;
pub mod bot_methods;
pub mod chat;
pub mod commands;
pub mod errors;
pub mod handlers;
pub mod message;
pub mod parse_json;
pub mod photo_size;
pub mod update;
pub mod user;

use bot::Bot;
use commands::Command;
use handlers::Handler;
use update::Update;

pub async fn run_bot<'a, U: Command + 'a, T: Handler<U> + 'a>(
    bot: &mut Bot,
    handler: &mut T,
) -> Result<(), Box<dyn std::error::Error + 'a>> {
    loop {
        let updates = bot.get_all_updates().await?;

        for update in updates.updates {
            let id = update.update_id;
            match handle_update(bot, handler, update).await {
                Ok(_) => {
                    bot.last_update = id;
                    log::info!("updated last processed update to {id}");
                }
                Err(err) => {
                    bot.last_update = id;
                    log::error!("Bot encountered error: {err}");
                }
            }
        }
    }
}

pub async fn handle_update<'a, U: Command + 'a, T: Handler<U> + 'a>(
    bot: &mut Bot,
    handler: &mut T,
    update: Update,
) -> Result<(), Box<dyn std::error::Error + 'a>> {
    log::info!("handling update (id {})", update.update_id);
    let msg = update.get_message()?;
    if msg.is_command() {
        let cmd = msg.get_command::<U>()?;
        handler.handle_command(bot, cmd, msg).await?;
    } else {
        handler.handle_message(bot, msg).await?;
    }

    Ok(())
}

pub mod test_common {
    use serde::Deserialize;
    use serde_json::from_str;
    use std::{fs, path::PathBuf};

    #[derive(Deserialize)]
    pub struct JSONData {
        pub api_key: String,
        pub white_list: Vec<i64>,
    }

    pub fn load_config() -> JSONData {
        let config_path = PathBuf::from("../../testing/bot_conf.json");
        let file_contents = fs::read_to_string(config_path).unwrap();
        let res: JSONData = from_str(&file_contents).unwrap();
        res
    }
}
