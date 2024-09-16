use super::{
    errors::{Error, WrongType},
    message::Message,
    parse_json::{check_ok, get_i64, get_map},
};
use serde_json::Value;

#[derive(Debug, PartialEq, Eq)]
pub struct Updates {
    pub updates: Vec<Update>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    pub update_id: i64,
    pub content: Option<UpdateContent>,
}

impl Update {
    pub fn get_message(&self) -> Result<Message, Error> {
        if let Some(UpdateContent::Message(msg)) = &self.content {
            Ok(msg.to_owned())
        } else {
            Err(Error::NoMessage(Box::new(self.clone())))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateContent {
    Message(Message),
    EditedMessage(Message),
}

impl TryFrom<Value> for Update {
    type Error = Error;
    fn try_from(val: Value) -> Result<Update, Self::Error> {
        let mut val_map = get_map(val)?;

        let update_id = get_i64(&mut val_map, "update_id")?;

        let mut content = None;

        let content_msg = val_map.remove("message");
        if let Some(msg_val) = content_msg {
            let msg = msg_val.try_into()?;
            content = Some(UpdateContent::Message(msg));
        }

        let content_edited = val_map.remove("edited_message");
        if let Some(msg_val) = content_edited {
            let msg = msg_val.try_into()?;
            content = Some(UpdateContent::EditedMessage(msg));
        }

        Ok(Update { update_id, content })
    }
}

impl TryFrom<Value> for Updates {
    type Error = Error;
    fn try_from(val: Value) -> Result<Updates, Self::Error> {
        let ok_val = check_ok(val)?;

        let update_vals = if let Value::Array(vals) = ok_val {
            Ok(vals)
        } else {
            Err(WrongType {
                field_name: "response".to_owned(),
                field_type: "array".to_owned(),
            })
        }?;

        let mut updates: Vec<Update> = vec![];
        for update_val in update_vals {
            let next_update: Update = update_val.try_into()?;
            updates.push(next_update);
        }

        Ok(Updates { updates })
    }
}

#[cfg(test)]
mod updates_test {
    use super::{Update, UpdateContent, Updates};
    use crate::{chat::Chat, message::Message};
    use serde_json::{Map, Number, Value};

    fn example_update_val() -> Map<String, Value> {
        let mut chat_map = Map::new();
        chat_map.insert("id".to_owned(), Value::Number(Number::from(1)));
        chat_map.insert("type".to_owned(), Value::String("message".to_owned()));

        let mut message_map = Map::new();
        message_map.insert("message_id".to_owned(), Value::Number(Number::from(1)));
        message_map.insert("date".to_owned(), Value::Number(Number::from(1)));
        message_map.insert("chat".to_owned(), Value::Object(chat_map));

        let mut val_map = Map::new();
        val_map.insert("update_id".to_owned(), Value::Number(Number::from(1)));
        val_map.insert("message".to_owned(), Value::Object(message_map));

        val_map
    }

    fn example_update() -> Update {
        Update {
            update_id: 1,
            content: Some(UpdateContent::Message(Message {
                id: 1,
                date: 1,
                chat: Chat {
                    id: 1,
                    ty: "message".to_owned(),
                    username: None,
                    first_name: None,
                    last_name: None,
                    title: None,
                },
                from: None,
                text: None,
                entities: None,
                photo: None,
                caption: None,
            })),
        }
    }

    #[test]
    fn update_from_val() {
        let result =
            <Value as TryInto<Update>>::try_into(Value::Object(example_update_val())).unwrap();
        let expected = example_update();
        assert_eq!(result, expected)
    }

    #[test]
    fn update_no_content() {
        let mut val_map = example_update_val();
        val_map.remove("message");
        let result = <Value as TryInto<Update>>::try_into(Value::Object(val_map)).unwrap();
        let mut expected = example_update();
        expected.content = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn update_edited() {
        let mut val_map = example_update_val();
        let msg_val = val_map.remove("message").unwrap();
        val_map.insert("edited_message".to_owned(), msg_val);
        let result = <Value as TryInto<Update>>::try_into(Value::Object(val_map)).unwrap();
        let mut expected = example_update();
        let msg = match expected.content.unwrap() {
            UpdateContent::Message(msg) => msg,
            UpdateContent::EditedMessage(_) => panic!("should never happen"),
        };
        expected.content = Some(UpdateContent::EditedMessage(msg));
        assert_eq!(result, expected)
    }

    #[test]
    fn update_no_map() {
        let result = <Value as TryInto<Update>>::try_into(Value::String("bad value".to_owned()));
        assert!(result.is_err())
    }

    #[test]
    fn update_no_id() {
        let mut val_map = example_update_val();
        val_map.remove("update_id");
        let result = <Value as TryInto<Update>>::try_into(Value::Object(val_map));
        assert!(result.is_err())
    }

    #[test]
    fn update_bad_msg() {
        let mut val_map = example_update_val();
        val_map.insert("message".to_owned(), Value::String("bad value".to_owned()));
        let result = <Value as TryInto<Update>>::try_into(Value::Object(val_map));
        assert!(result.is_err())
    }

    #[test]
    fn update_bad_edited() {
        let mut val_map = example_update_val();
        val_map.remove("message");
        val_map.insert(
            "edited_message".to_owned(),
            Value::String("bad value".to_owned()),
        );
        let result = <Value as TryInto<Update>>::try_into(Value::Object(val_map));
        assert!(result.is_err())
    }

    fn example_updates_val() -> Map<String, Value> {
        let mut val_map = Map::new();
        val_map.insert("ok".to_owned(), Value::Bool(true));
        val_map.insert(
            "result".to_owned(),
            Value::Array(vec![Value::Object(example_update_val())]),
        );
        val_map
    }

    #[test]
    fn updates_from_val() {
        let result =
            <Value as TryInto<Updates>>::try_into(Value::Object(example_updates_val())).unwrap();
        let expected = Updates {
            updates: vec![example_update()],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn updates_no_map() {
        let result = <Value as TryInto<Updates>>::try_into(Value::String("bad value".to_owned()));
        assert!(result.is_err())
    }

    #[test]
    fn updates_no_array() {
        let mut val_map = example_updates_val();
        val_map.insert("result".to_owned(), Value::String("bad value".to_owned()));
        let result = <Value as TryInto<Updates>>::try_into(Value::Object(val_map));
        assert!(result.is_err())
    }

    #[test]
    fn updates_bad_update() {
        let mut val_map = example_updates_val();
        val_map.insert(
            "result".to_owned(),
            Value::Array(vec![Value::String("bad value".to_owned())]),
        );
        let result = <Value as TryInto<Updates>>::try_into(Value::Object(val_map));
        assert!(result.is_err())
    }
}
