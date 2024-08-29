use super::{
    chat::Chat,
    commands::Command,
    errors::Error,
    parse_json::{get_i64, get_map, get_option_array, get_option_str, get_str, get_val},
    photo_size::Photo,
    user::User,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub date: i64,
    pub from: Option<User>,
    pub chat: Chat,
    pub text: Option<String>,
    pub photo: Option<Photo>,
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
            Err(Error::CommandIsMessage(self.to_owned()))
        }
    }

    pub fn get_text(&self) -> Result<String, Error> {
        self.text
            .clone()
            .ok_or(Error::EmptyMessage(self.to_owned()))
    }

    pub fn get_command<'a, U: Command + 'a>(&self) -> Result<U, Box<dyn std::error::Error + 'a>> {
        self.ensure_command()?;
        let msg_text = self.get_text()?;
        let cmd = msg_text
            .split(' ')
            .next()
            .ok_or(Error::CommandIsMessage(self.to_owned()))?;
        U::parse(cmd).map_err(|err| err.into())
    }
}

#[derive(Debug, Clone)]
pub struct MessageEntity {
    ty: String,
    offset: i64,
    length: i64,
    url: Option<String>,
    user: Option<User>,
    language: Option<String>,
    custom_emoji_id: Option<String>,
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
        let url = get_option_str(&mut val_map, "str")?;
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
