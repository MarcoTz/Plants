use super::{bot::Bot, commands::Command, message::Message};

pub trait Handler<T: Command> {
    type Error: std::error::Error;
    async fn handle_message(&mut self, bot: &Bot, message: Message) -> Result<(), Self::Error>;
    async fn handle_command(
        &mut self,
        bot: &Bot,
        cmd: T,
        message: Message,
    ) -> Result<(), Self::Error>;

    async fn handle<'a>(
        &mut self,
        message: Message,
        bot: &Bot,
    ) -> Result<(), Box<dyn std::error::Error + 'a>>
    where
        T: 'a,
        Self: 'a,
    {
        if message.is_command() {
            match message.get_command::<T>() {
                Ok(cmd) => {
                    self.handle_command(bot, cmd, message).await?;
                }
                Err(err) => {
                    log::error!("{err}");
                }
            }
            Ok(())
        } else {
            self.handle_message(bot, message).await?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod handler_tests {
    use super::Handler;
    use crate::{
        bot::Bot,
        chat::Chat,
        message::{Message, MessageEntity},
        test_common::ExampleHandler,
    };

    #[tokio::test]
    async fn handle_message() {
        let example_message = Message {
            id: 1,
            date: 1,
            from: None,
            chat: Chat {
                id: 1,
                ty: "message".to_owned(),
                title: None,
                username: None,
                first_name: None,
                last_name: None,
            },
            text: Some("test".to_owned()),
            photo: None,
            caption: None,
            entities: None,
        };
        let mut handler = ExampleHandler {};
        let bot = Bot::new("sample_key".to_owned());

        let result = handler.handle(example_message, &bot).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn handle_command() {
        let example_message = Message {
            id: 1,
            date: 1,
            from: None,
            chat: Chat {
                id: 1,
                ty: "message".to_owned(),
                title: None,
                username: None,
                first_name: None,
                last_name: None,
            },
            text: Some("/succ".to_owned()),
            photo: None,
            caption: None,
            entities: Some(vec![MessageEntity {
                ty: "bot_command".to_string(),
                offset: 1,
                length: 1,
                url: None,
                user: None,
                language: None,
                custom_emoji_id: None,
            }]),
        };
        let mut handler = ExampleHandler {};
        let bot = Bot::new("sample_key".to_owned());

        let result = handler.handle(example_message, &bot).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn handle_command_no_parse() {
        let example_message = Message {
            id: 1,
            date: 1,
            from: None,
            chat: Chat {
                id: 1,
                ty: "message".to_owned(),
                title: None,
                username: None,
                first_name: None,
                last_name: None,
            },
            text: Some("/something".to_owned()),
            photo: None,
            caption: None,
            entities: Some(vec![MessageEntity {
                ty: "bot_command".to_string(),
                offset: 1,
                length: 1,
                url: None,
                user: None,
                language: None,
                custom_emoji_id: None,
            }]),
        };
        let mut handler = ExampleHandler {};
        let bot = Bot::new("sample_key".to_owned());

        let result = handler.handle(example_message, &bot).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn handle_command_handle_err() {
        let example_message = Message {
            id: 1,
            date: 1,
            from: None,
            chat: Chat {
                id: 1,
                ty: "message".to_owned(),
                title: None,
                username: None,
                first_name: None,
                last_name: None,
            },
            text: Some("/err".to_owned()),
            photo: None,
            caption: None,
            entities: Some(vec![MessageEntity {
                ty: "bot_command".to_string(),
                offset: 1,
                length: 1,
                url: None,
                user: None,
                language: None,
                custom_emoji_id: None,
            }]),
        };
        let mut handler = ExampleHandler {};
        let bot = Bot::new("sample_key".to_owned());

        let result = handler.handle(example_message, &bot).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn handle_message_err() {
        let example_message = Message {
            id: 1,
            date: 1,
            from: None,
            chat: Chat {
                id: 1,
                ty: "message".to_owned(),
                title: None,
                username: None,
                first_name: None,
                last_name: None,
            },
            text: None,
            photo: None,
            caption: None,
            entities: None,
        };
        let mut handler = ExampleHandler {};
        let bot = Bot::new("sample_key".to_owned());

        let result = handler.handle(example_message, &bot).await;
        assert!(result.is_err())
    }
}
