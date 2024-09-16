use super::{
    chat::Chat,
    commands::Command,
    errors::Error,
    parse_json::{get_i64, get_map, get_option_array, get_option_str, get_str, get_val},
    photo_size::Photo,
    user::User,
};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: i64,
    pub date: i64,
    pub from: Option<User>,
    pub chat: Chat,
    pub text: Option<String>,
    pub photo: Option<Photo>,
    pub caption: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
}

impl Message {
    pub fn is_command(&self) -> bool {
        match &self.entities {
            Some(entities) => {
                let mut is_cmd = false;
                for entity in entities {
                    is_cmd = is_cmd || entity.ty == "bot_command";
                }
                is_cmd
            }
            None => false,
        }
    }

    pub fn ensure_command(&self) -> Result<(), Error> {
        if self.is_command() {
            Ok(())
        } else {
            Err(Error::CommandIsMessage)
        }
    }

    pub fn get_text(&self) -> Result<String, Error> {
        self.text.clone().ok_or(Error::EmptyMessage)
    }

    pub fn get_command<'a, U: Command + 'a>(&self) -> Result<U, Box<dyn std::error::Error + 'a>> {
        self.ensure_command()?;
        let msg_text = self.get_text()?;
        let cmd = msg_text.split(' ').next().ok_or(Error::CommandIsMessage)?;
        U::parse(&cmd.replace('/', "")).map_err(|err| err.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageEntity {
    pub ty: String,
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}

impl TryFrom<Value> for Message {
    type Error = Error;
    fn try_from(val: Value) -> Result<Message, Self::Error> {
        let mut val_map = get_map(val)?;
        let id = get_i64(&mut val_map, "message_id")?;

        let mut from = None;
        let from_val = val_map.remove("from");
        if let Some(user_val) = from_val {
            let user = user_val.try_into()?;
            from = Some(user);
        }

        let chat_val = get_val(&mut val_map, "chat")?;
        let chat = chat_val.try_into()?;

        let date = get_i64(&mut val_map, "date")?;

        let text = get_option_str(&mut val_map, "text")?;

        let mut photo = None;
        let photo_val = val_map.remove("photo");
        if let Some(val) = photo_val {
            let ph = val.try_into()?;
            photo = Some(ph);
        }

        let mut caption = None;
        let caption_val = val_map.remove("caption");
        if let Some(Value::String(val)) = caption_val {
            caption = Some(val);
        }

        let mut entities = None;
        let entity_val = get_option_array(&mut val_map, "entities")?;
        if let Some(vals) = entity_val {
            let mut entity_vec = vec![];
            for val in vals {
                let new_entity = val.try_into()?;
                entity_vec.push(new_entity);
            }
            entities = Some(entity_vec);
        }

        Ok(Message {
            id,
            from,
            chat,
            date,
            text,
            photo,
            caption,
            entities,
        })
    }
}

impl TryFrom<Value> for MessageEntity {
    type Error = Error;

    fn try_from(val: Value) -> Result<MessageEntity, Self::Error> {
        let mut val_map = get_map(val)?;
        let ty = get_str(&mut val_map, "type")?;
        let offset = get_i64(&mut val_map, "offset")?;
        let length = get_i64(&mut val_map, "length")?;
        let url = get_option_str(&mut val_map, "url")?;
        let mut user = None;
        let user_val = val_map.remove("user");
        if let Some(val) = user_val {
            let usr = val.try_into()?;
            user = Some(usr);
        }
        let language = get_option_str(&mut val_map, "language")?;
        let custom_emoji_id = get_option_str(&mut val_map, "custom_emoji_id")?;
        Ok(MessageEntity {
            ty,
            offset,
            length,
            url,
            user,
            language,
            custom_emoji_id,
        })
    }
}

#[cfg(test)]
mod message_tests {
    use super::{Message, MessageEntity};
    use crate::{
        chat::Chat,
        photo_size::{Photo, PhotoSize},
        test_common::ExampleCommand,
        user::User,
    };
    use serde_json::{Map, Number, Value};

    fn example_message() -> Message {
        Message {
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
            text: Some("a message".to_owned()),
            photo: None,
            caption: None,
            entities: None,
        }
    }
    fn example_command() -> Message {
        Message {
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
            entities: Some(vec![MessageEntity {
                ty: "bot_command".to_owned(),
                offset: 1,
                length: 1,
                url: None,
                user: None,
                language: None,
                custom_emoji_id: None,
            }]),
        }
    }

    #[test]
    fn msg_is_command() {
        let result = example_message().is_command();
        assert!(!result)
    }

    #[test]
    fn cmd_is_command() {
        let result = example_command().is_command();
        assert!(result)
    }

    #[test]
    fn ensure_command_msg() {
        assert!(example_message().ensure_command().is_err())
    }

    #[test]
    fn ensure_command_cmd() {
        assert!(example_command().ensure_command().is_ok())
    }

    #[test]
    fn get_message_msg() {
        let result = example_message().get_text().unwrap();
        assert_eq!(result, "a message".to_owned())
    }

    #[test]
    fn get_message_cmd() {
        let result = example_command().get_text();
        assert!(result.is_err())
    }

    #[test]
    fn get_command_msg() {
        let result = example_message().get_command::<ExampleCommand>();
        assert!(result.is_err())
    }

    #[test]
    fn get_command_no_text() {
        let result = example_command().get_command::<ExampleCommand>();
        assert!(result.is_err())
    }

    #[test]
    fn get_command_no_content() {
        let example_command = Message {
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
            text: Some("".to_owned()),
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
        };
        let result = example_command.get_command::<ExampleCommand>();
        assert!(result.is_err())
    }

    #[test]
    fn get_command_cmd() {
        let example_command = Message {
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
                ty: "bot_command".to_owned(),
                offset: 1,
                length: 1,
                url: None,
                user: None,
                language: None,
                custom_emoji_id: None,
            }]),
        };
        let result = example_command.get_command::<ExampleCommand>().unwrap();
        assert_eq!(result, ExampleCommand::Succ)
    }

    fn example_user_value() -> Value {
        let mut example_user = Map::new();
        example_user.insert("id".to_owned(), Value::Number(Number::from(1)));
        example_user.insert("is_bot".to_owned(), Value::Bool(false));
        example_user.insert("first_name".to_owned(), Value::String("name".to_owned()));
        Value::Object(example_user)
    }

    fn example_user_parsed() -> User {
        User {
            id: 1,
            is_bot: false,
            first_name: "name".to_owned(),
            last_name: None,
            username: None,
            language_code: None,
        }
    }

    fn example_value() -> Map<String, Value> {
        let mut example_chat = Map::new();
        example_chat.insert("id".to_owned(), Value::Number(Number::from(1)));
        example_chat.insert("type".to_owned(), Value::String("message".to_owned()));

        let mut example_entity = Map::new();
        example_entity.insert("type".to_owned(), Value::String("bot_command".to_owned()));
        example_entity.insert("offset".to_owned(), Value::Number(Number::from(1)));
        example_entity.insert("length".to_owned(), Value::Number(Number::from(1)));

        let mut example_photo_map = Map::new();
        example_photo_map.insert("file_id".to_owned(), Value::String("photo_id".to_owned()));
        example_photo_map.insert(
            "file_unique_id".to_owned(),
            Value::String("unique_id".to_owned()),
        );
        example_photo_map.insert("width".to_owned(), Value::Number(Number::from(10)));
        example_photo_map.insert("height".to_owned(), Value::Number(Number::from(10)));

        let mut example_map = Map::new();
        example_map.insert("message_id".to_owned(), Value::Number(Number::from(1)));
        example_map.insert("from".to_owned(), example_user_value());
        example_map.insert("chat".to_owned(), Value::Object(example_chat));
        example_map.insert("date".to_owned(), Value::Number(Number::from(1)));
        example_map.insert("text".to_owned(), Value::String("a message".to_owned()));
        example_map.insert(
            "photo".to_owned(),
            Value::Array(vec![Value::Object(example_photo_map)]),
        );
        example_map.insert(
            "entities".to_owned(),
            Value::Array(vec![Value::Object(example_entity)]),
        );
        example_map
    }

    fn example_parsed() -> Message {
        Message {
            id: 1,
            from: Some(example_user_parsed()),
            chat: Chat {
                id: 1,
                ty: "message".to_owned(),
                title: None,
                username: None,
                first_name: None,
                last_name: None,
            },
            date: 1,
            text: Some("a message".to_owned()),
            photo: Some(Photo {
                sizes: vec![PhotoSize {
                    file_id: "photo_id".to_owned(),
                    file_unique_id: "unique_id".to_owned(),
                    width: 10,
                    height: 10,
                    file_size: None,
                }],
            }),
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
        }
    }

    #[test]
    fn from_val() {
        let result = <Value as TryInto<Message>>::try_into(Value::Object(example_value())).unwrap();
        let expected = example_parsed();
        assert_eq!(result, expected)
    }

    #[test]
    fn from_val_no_from() {
        let mut val = example_value();
        val.remove("from");
        let mut message = example_parsed();
        message.from = None;
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val)).unwrap();
        assert_eq!(result, message)
    }

    #[test]
    fn from_val_no_text() {
        let mut val = example_value();
        val.remove("text");
        let mut message = example_parsed();
        message.text = None;
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val)).unwrap();
        assert_eq!(result, message)
    }

    #[test]
    fn from_val_no_photo() {
        let mut val = example_value();
        val.remove("photo");
        let mut message = example_parsed();
        message.photo = None;
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val)).unwrap();
        assert_eq!(result, message)
    }

    #[test]
    fn from_val_no_entities() {
        let mut val = example_value();
        val.remove("entities");
        let mut message = example_parsed();
        message.entities = None;
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val)).unwrap();
        assert_eq!(result, message)
    }

    #[test]
    fn from_val_no_id() {
        let mut val = example_value();
        val.remove("message_id");
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn from_val_no_chat() {
        let mut val = example_value();
        val.remove("chat");
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn from_val_no_date() {
        let mut val = example_value();
        val.remove("date");
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn from_val_no_map() {
        let result = <Value as TryInto<Message>>::try_into(Value::String("cannot work".to_owned()));
        assert!(result.is_err())
    }

    #[test]
    fn from_val_bad_from() {
        let mut val = example_value();
        val.insert("from".to_owned(), Value::String("bad value".to_owned()));
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn from_val_bad_chat() {
        let mut val = example_value();
        val.insert("chat".to_owned(), Value::String("bad value".to_owned()));
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn from_val_bad_text() {
        let mut val = example_value();
        val.insert("text".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn from_val_bad_photo() {
        let mut val = example_value();
        val.insert("photo".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn from_val_bad_entitites() {
        let mut val = example_value();
        val.insert("entities".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn from_val_bad_entitiy() {
        let mut val = example_value();
        val.insert(
            "entities".to_owned(),
            Value::Array(vec![Value::String("bad Value".to_owned())]),
        );
        let result = <Value as TryInto<Message>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    fn example_entity_val() -> Map<String, Value> {
        let mut entity_map = Map::new();
        entity_map.insert("type".to_owned(), Value::String("bot_command".to_owned()));
        entity_map.insert("offset".to_owned(), Value::Number(Number::from(1)));
        entity_map.insert("length".to_owned(), Value::Number(Number::from(1)));
        entity_map.insert("url".to_owned(), Value::String("a url".to_owned()));
        entity_map.insert("user".to_owned(), example_user_value());
        entity_map.insert("language".to_owned(), Value::String("en".to_owned()));
        entity_map.insert(
            "custom_emoji_id".to_owned(),
            Value::String("emoji".to_owned()),
        );
        entity_map
    }

    fn example_entity_parsed() -> MessageEntity {
        MessageEntity {
            ty: "bot_command".to_owned(),
            offset: 1,
            length: 1,
            url: Some("a url".to_owned()),
            user: Some(example_user_parsed()),
            language: Some("en".to_owned()),
            custom_emoji_id: Some("emoji".to_owned()),
        }
    }

    #[test]
    fn entity_from_val() {
        let result =
            <Value as TryInto<MessageEntity>>::try_into(Value::Object(example_entity_val()))
                .unwrap();
        let expected = example_entity_parsed();
        assert_eq!(result, expected);
    }

    #[test]
    fn entity_from_val_no_url() {
        let mut val = example_entity_val();
        val.remove("url");
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val)).unwrap();
        let mut expected = example_entity_parsed();
        expected.url = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn entity_from_val_no_user() {
        let mut val = example_entity_val();
        val.remove("user");
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val)).unwrap();
        let mut expected = example_entity_parsed();
        expected.user = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn entity_from_val_no_language() {
        let mut val = example_entity_val();
        val.remove("language");
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val)).unwrap();
        let mut expected = example_entity_parsed();
        expected.language = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn entity_from_val_no_emoji() {
        let mut val = example_entity_val();
        val.remove("custom_emoji_id");
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val)).unwrap();
        let mut expected = example_entity_parsed();
        expected.custom_emoji_id = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn entity_from_val_no_map() {
        let result =
            <Value as TryInto<MessageEntity>>::try_into(Value::String("bad value".to_owned()));
        assert!(result.is_err())
    }

    #[test]
    fn entity_bad_ty() {
        let mut val = example_entity_val();
        val.insert("type".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn entity_bad_offset() {
        let mut val = example_entity_val();
        val.insert("offset".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn entity_bad_length() {
        let mut val = example_entity_val();
        val.insert("length".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn entity_bad_url() {
        let mut val = example_entity_val();
        val.insert("url".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn entity_bad_user() {
        let mut val = example_entity_val();
        val.insert("user".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn entity_bad_language() {
        let mut val = example_entity_val();
        val.insert("language".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }

    #[test]
    fn entity_bad_emoji() {
        let mut val = example_entity_val();
        val.insert("custom_emoji_id".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<MessageEntity>>::try_into(Value::Object(val));
        assert!(result.is_err())
    }
}
