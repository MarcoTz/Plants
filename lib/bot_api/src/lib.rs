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
use handlers::{CommandHandler, ErrorHandler, MessageHandler};

async fn handle_updates<'a, V: Command + 'a, T: MessageHandler + 'a + CommandHandler<V>>(
    bot: &mut Bot,
    handler: &mut T,
) -> Result<(), Box<dyn std::error::Error + 'a>> {
    let updates = bot.get_all_updates().await?;
    for update in updates.updates {
        let msg = update.get_message()?;
        if msg.is_command() {
            handler.handle_command(msg, bot).await?;
        } else {
            handler.handle_message(msg, bot).await?;
        }
    }
    Ok(())
}

pub async fn run_bot<
    'a,
    W: ErrorHandler<'a>,
    U: Command + 'a,
    T: MessageHandler + 'a + CommandHandler<U>,
>(
    bot: &mut Bot,
    handler: &mut T,
    err_handler: &W,
) {
    loop {
        if let Err(err) = handle_updates(bot, handler).await {
            err_handler.handle_error(err)
        }
    }
}
