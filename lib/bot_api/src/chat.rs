use super::{
    errors::Error,
    parse_json::{get_i64, get_map, get_option_str, get_str},
};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chat {
    pub id: i64,
    pub ty: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl TryFrom<Value> for Chat {
    type Error = Error;
    fn try_from(val: Value) -> Result<Chat, Self::Error> {
        let mut val_map = get_map(val)?;
        let id = get_i64(&mut val_map, "id")?;
        let ty = get_str(&mut val_map, "type")?;
        let title = get_option_str(&mut val_map, "title")?;
        let username = get_option_str(&mut val_map, "username")?;
        let first_name = get_option_str(&mut val_map, "first_name")?;
        let last_name = get_option_str(&mut val_map, "last_name")?;

        Ok(Chat {
            id,
            ty,
            title,
            username,
            first_name,
            last_name,
        })
    }
}

#[cfg(test)]
mod chat_tests {
    use super::Chat;
    use serde_json::{Map, Number, Value};

    fn example_val() -> Map<String, Value> {
        let mut example_map = Map::new();
        example_map.insert("id".to_owned(), Value::Number(Number::from(1)));
        example_map.insert("type".to_owned(), Value::String("chat".to_owned()));
        example_map.insert("title".to_owned(), Value::String("test chat".to_owned()));
        example_map.insert("username".to_owned(), Value::String("some user".to_owned()));
        example_map.insert(
            "first_name".to_owned(),
            Value::String("firstname".to_owned()),
        );
        example_map.insert("last_name".to_owned(), Value::String("lastname".to_owned()));
        example_map
    }

    fn example_chat() -> Chat {
        Chat {
            id: 1,
            ty: "chat".to_owned(),
            title: Some("test chat".to_owned()),
            username: Some("some user".to_owned()),
            first_name: Some("firstname".to_owned()),
            last_name: Some("lastname".to_owned()),
        }
    }

    #[test]
    fn from_val() {
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val())).unwrap();
        assert_eq!(result, example_chat())
    }

    #[test]
    fn no_title() {
        let mut example_val = example_val();
        example_val.remove("title");
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val)).unwrap();
        let mut expected = example_chat();
        expected.title = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn no_username() {
        let mut example_val = example_val();
        example_val.remove("username");
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val)).unwrap();
        let mut expected = example_chat();
        expected.username = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn no_firstname() {
        let mut example_val = example_val();
        example_val.remove("first_name");
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val)).unwrap();
        let mut expected = example_chat();
        expected.first_name = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn no_lastname() {
        let mut example_val = example_val();
        example_val.remove("last_name");
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val)).unwrap();
        let mut expected = example_chat();
        expected.last_name = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn no_id() {
        let mut example_val = example_val();
        example_val.remove("id");
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val));
        assert!(result.is_err())
    }

    #[test]
    fn no_ty() {
        let mut example_val = example_val();
        example_val.remove("type");
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val));
        assert!(result.is_err())
    }

    #[test]
    fn no_map() {
        let result =
            <Value as TryInto<Chat>>::try_into(Value::String("not a valid map".to_owned()));
        assert!(result.is_err())
    }

    #[test]
    fn title_no_str() {
        let mut example_val = example_val();
        example_val.insert("title".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val));
        assert!(result.is_err())
    }

    #[test]
    fn username_no_str() {
        let mut example_val = example_val();
        example_val.insert("username".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val));
        assert!(result.is_err())
    }

    #[test]
    fn firstname_no_str() {
        let mut example_val = example_val();
        example_val.insert("first_name".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val));
        assert!(result.is_err())
    }

    #[test]
    fn lastname_no_str() {
        let mut example_val = example_val();
        example_val.insert("last_name".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<Chat>>::try_into(Value::Object(example_val));
        assert!(result.is_err())
    }
}
