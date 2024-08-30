use super::{
    bot_methods::{BotMethod, GetUpdates, SendMessage},
    errors::Error,
    update::Updates,
};

pub struct Bot {
    pub api_key: String,
    pub last_update: i64,
}

impl Bot {
    pub fn new(api_key: String) -> Bot {
        Bot {
            api_key,
            last_update: 0,
        }
    }

    pub async fn get_updates(
        &mut self,
        limit: Option<i32>,
        timeout: Option<i32>,
        allowed_updates: Option<Vec<String>>,
    ) -> Result<Updates, Error> {
        let update = GetUpdates {
            offset: Some(self.last_update + 1),
            limit,
            timeout,
            allowed_updates,
        };
        let mut updates = update.perform(&self.api_key).await?;
        updates
            .updates
            .retain(|upd| upd.update_id > self.last_update);

        Ok(updates)
    }

    pub async fn get_all_updates(&mut self) -> Result<Updates, Error> {
        self.get_updates(None, Some(1), None).await
    }

    pub async fn send_message(&self, chat_id: String, text: String) -> Result<(), Error> {
        SendMessage { chat_id, text }.perform(&self.api_key).await?;
        Ok(())
    }
}
