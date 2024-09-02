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
