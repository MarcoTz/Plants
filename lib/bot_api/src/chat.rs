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
