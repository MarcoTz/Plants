use super::{
    errors::Error,
    parse_json::{get_bool, get_i64, get_map, get_option_str, get_str},
};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

impl TryFrom<Value> for User {
    type Error = Error;
    fn try_from(val: Value) -> Result<User, Error> {
        let mut val_map = get_map(val)?;
        let id = get_i64(&mut val_map, "id")?;
        let is_bot = get_bool(&mut val_map, "is_bot")?;
        let first_name = get_str(&mut val_map, "first_name")?;
        let last_name = get_option_str(&mut val_map, "last_name")?;
        let username = get_option_str(&mut val_map, "username")?;
        let language_code = get_option_str(&mut val_map, "language_code")?;
        Ok(User {
            id,
            is_bot,
            first_name,
            last_name,
            username,
            language_code,
        })
    }
}

#[cfg(test)]
mod user_tests {
    use super::User;
    use serde_json::{Map, Number, Value};

    fn example_user_val() -> Map<String, Value> {
        let mut val_map = Map::new();
        val_map.insert("id".to_owned(), Value::Number(Number::from(1)));
        val_map.insert("is_bot".to_owned(), Value::Bool(false));
        val_map.insert(
            "first_name".to_owned(),
            Value::String("firstname".to_owned()),
        );
        val_map.insert("last_name".to_owned(), Value::String("lastname".to_owned()));
        val_map.insert("username".to_owned(), Value::String("username".to_owned()));
        val_map.insert("language_code".to_owned(), Value::String("en".to_owned()));
        val_map
    }

    fn example_user_parsed() -> User {
        User {
            id: 1,
            is_bot: false,
            first_name: "firstname".to_owned(),
            last_name: Some("lastname".to_owned()),
            username: Some("username".to_owned()),
            language_code: Some("en".to_owned()),
        }
    }

    #[test]
    fn user_from_val() {
        let result = <Value as TryInto<User>>::try_into(Value::Object(example_user_val())).unwrap();
        let expected = example_user_parsed();
        assert_eq!(result, expected)
    }

    #[test]
    fn user_no_lastname() {
        let mut user_map = example_user_val();
        user_map.remove("last_name");
        let result = <Value as TryInto<User>>::try_into(Value::Object(user_map)).unwrap();
        let mut expected = example_user_parsed();
        expected.last_name = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn user_no_username() {
        let mut user_map = example_user_val();
        user_map.remove("username");
        let result = <Value as TryInto<User>>::try_into(Value::Object(user_map)).unwrap();
        let mut expected = example_user_parsed();
        expected.username = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn user_no_lang() {
        let mut user_map = example_user_val();
        user_map.remove("language_code");
        let result = <Value as TryInto<User>>::try_into(Value::Object(user_map)).unwrap();
        let mut expected = example_user_parsed();
        expected.language_code = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn user_no_id() {
        let mut user_map = example_user_val();
        user_map.remove("id");
        let result = <Value as TryInto<User>>::try_into(Value::Object(user_map));
        assert!(result.is_err())
    }

    #[test]
    fn user_no_bot() {
        let mut user_map = example_user_val();
        user_map.remove("is_bot");
        let result = <Value as TryInto<User>>::try_into(Value::Object(user_map));
        assert!(result.is_err())
    }

    #[test]
    fn user_no_firstname() {
        let mut user_map = example_user_val();
        user_map.remove("first_name");
        let result = <Value as TryInto<User>>::try_into(Value::Object(user_map));
        assert!(result.is_err())
    }

    #[test]
    fn user_no_map() {
        let result = <Value as TryInto<User>>::try_into(Value::String("bad value".to_owned()));
        assert!(result.is_err())
    }

    #[test]
    fn user_bad_lastname() {
        let mut user_map = example_user_val();
        user_map.insert("last_name".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<User>>::try_into(Value::Object(user_map));
        assert!(result.is_err())
    }

    #[test]
    fn user_bad_username() {
        let mut user_map = example_user_val();
        user_map.insert("username".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<User>>::try_into(Value::Object(user_map));
        assert!(result.is_err())
    }

    #[test]
    fn user_bad_lang() {
        let mut user_map = example_user_val();
        user_map.insert("language_code".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<User>>::try_into(Value::Object(user_map));
        assert!(result.is_err())
    }
}
