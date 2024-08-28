use super::{
    chat::Chat,
    errors::Error,
    parse_json::{get_i64, get_map, get_option_str, get_val},
    photo_size::Photo,
    user::User,
};
use serde_json::Value;

pub struct Message {
    id: i64,
    date: i64,
    from: Option<User>,
    chat: Chat,
    text: Option<String>,
    photo: Option<Photo>,
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

        Ok(Message {
            id,
            from,
            chat,
            date,
            text,
            photo,
        })
    }
}
