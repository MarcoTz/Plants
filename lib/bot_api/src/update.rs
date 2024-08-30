use super::{
    errors::{Error, WrongType},
    message::Message,
    parse_json::{check_ok, get_array, get_i64, get_map},
};
use serde_json::Value;

#[derive(Debug)]
pub struct Updates {
    pub updates: Vec<Update>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
            content = Some(UpdateContent::Message(msg));
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
