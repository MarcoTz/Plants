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

#[cfg(test)]
pub mod test_common {
    use crate::{bot::Bot, commands::Command, handlers::Handler, message::Message};
    use serde::Deserialize;
    use serde_json::from_str;
    use std::{fmt, fs, path::PathBuf};

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

    #[test]
    fn load_config_test() {
        load_config();
    }

    pub struct ExampleHandler;
    #[derive(Debug, PartialEq, Eq)]
    pub enum ExampleCommand {
        Succ,
        Error,
    }
    #[derive(Debug)]
    pub struct ExampleError;
    impl fmt::Display for ExampleError {
        fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
            frmt.write_str("example")
        }
    }

    #[test]
    fn display_example_err() {
        let result = format!("{}", ExampleError {});
        let expected = "example";
        assert_eq!(result, expected)
    }

    impl std::error::Error for ExampleError {}

    impl Command for ExampleCommand {
        type Error = ExampleError;
        fn parse(s: &str) -> Result<ExampleCommand, ExampleError> {
            match s {
                "succ" => Ok(ExampleCommand::Succ),
                "err" => Ok(ExampleCommand::Error),
                _ => Err(ExampleError),
            }
        }
        fn get_description(&self) -> String {
            panic!("not implemented")
        }
    }

    #[test]
    #[should_panic]
    fn example_command_description() {
        ExampleCommand::Succ.get_description();
    }

    impl Handler<ExampleCommand> for ExampleHandler {
        type Error = ExampleError;
        async fn handle_message(&mut self, _: &Bot, msg: Message) -> Result<(), ExampleError> {
            match msg.get_text() {
                Ok(_) => Ok(()),
                Err(_) => Err(ExampleError),
            }
        }
        async fn handle_command(
            &mut self,
            _: &Bot,
            cmd: ExampleCommand,
            _: Message,
        ) -> Result<(), ExampleError> {
            match cmd {
                ExampleCommand::Succ => Ok(()),
                ExampleCommand::Error => Err(ExampleError),
            }
        }
    }
}

#[cfg(test)]
mod api_tests {
    use super::{handle_update, run_bot};
    use crate::{
        bot::Bot,
        chat::Chat,
        message::{Message, MessageEntity},
        test_common::{load_config, ExampleHandler},
        update::{Update, UpdateContent},
    };

    fn example_bot() -> Bot {
        let data = load_config();
        Bot::new(data.api_key)
    }

    #[tokio::test]
    async fn handle_no_msg() {
        let example_update = Update {
            update_id: 1,
            content: None,
        };
        let mut bot = example_bot();
        let result = handle_update(&mut bot, &mut ExampleHandler {}, example_update).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn handle_command() {
        let mut bot = example_bot();
        let example_update = Update {
            update_id: 1,
            content: Some(UpdateContent::Message(Message {
                id: 1,
                date: 1,
                from: None,
                caption: None,
                chat: Chat {
                    id: 1,
                    ty: "type".to_owned(),
                    title: None,
                    username: None,
                    first_name: None,
                    last_name: None,
                },
                text: Some("/succ".to_owned()),
                photo: None,
                entities: Some(vec![MessageEntity {
                    ty: "bot_command".to_owned(),
                    offset: 1,
                    length: 1,
                    url: None,
                    user: None,
                    language: None,
                    custom_emoji_id: None,
                }]),
            })),
        };
        let result = handle_update(&mut bot, &mut ExampleHandler {}, example_update).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn handle_command_no_parse() {
        let mut bot = example_bot();
        let example_update = Update {
            update_id: 1,
            content: Some(UpdateContent::Message(Message {
                id: 1,
                date: 1,
                from: None,
                chat: Chat {
                    id: 1,
                    ty: "type".to_owned(),
                    title: None,
                    username: None,
                    first_name: None,
                    last_name: None,
                },
                text: Some("/something".to_owned()),
                photo: None,
                caption: None,
                entities: Some(vec![MessageEntity {
                    ty: "bot_command".to_owned(),
                    offset: 1,
                    length: 1,
                    url: None,
                    user: None,
                    language: None,
                    custom_emoji_id: None,
                }]),
            })),
        };
        let result = handle_update(&mut bot, &mut ExampleHandler {}, example_update).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn handle_message() {
        let mut bot = example_bot();
        let example_update = Update {
            update_id: 1,
            content: Some(UpdateContent::Message(Message {
                id: 1,
                date: 1,
                from: None,
                chat: Chat {
                    id: 1,
                    ty: "type".to_owned(),
                    title: None,
                    username: None,
                    first_name: None,
                    last_name: None,
                },
                text: Some("message".to_owned()),
                photo: None,
                caption: None,
                entities: None,
            })),
        };
        let result = handle_update(&mut bot, &mut ExampleHandler {}, example_update).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn handle_command_err() {
        let mut bot = example_bot();
        let example_update = Update {
            update_id: 1,
            content: Some(UpdateContent::Message(Message {
                id: 1,
                date: 1,
                from: None,
                chat: Chat {
                    id: 1,
                    ty: "type".to_owned(),
                    title: None,
                    username: None,
                    first_name: None,
                    last_name: None,
                },
                text: Some("/err".to_owned()),
                photo: None,
                caption: None,
                entities: Some(vec![MessageEntity {
                    ty: "bot_command".to_owned(),
                    offset: 1,
                    length: 1,
                    url: None,
                    user: None,
                    language: None,
                    custom_emoji_id: None,
                }]),
            })),
        };
        let result = handle_update(&mut bot, &mut ExampleHandler {}, example_update).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn handle_message_err() {
        let mut bot = example_bot();
        let example_update = Update {
            update_id: 1,
            content: Some(UpdateContent::Message(Message {
                id: 1,
                date: 1,
                from: None,
                chat: Chat {
                    id: 1,
                    ty: "type".to_owned(),
                    title: None,
                    username: None,
                    first_name: None,
                    last_name: None,
                },
                text: None,
                photo: None,
                caption: None,
                entities: None,
            })),
        };
        let result = handle_update(&mut bot, &mut ExampleHandler {}, example_update).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn run_err() {
        let mut bot = Bot::new("not a valid key".to_owned());
        let res = run_bot(&mut bot, &mut ExampleHandler {}).await;
        assert!(res.is_err())
    }
}
