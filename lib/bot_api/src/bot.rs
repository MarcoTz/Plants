use super::{
    bot_methods::{BotMethod, GetUpdates, SendMessage},
    commands::Command,
    errors::Error,
    handlers::{CommandHandler, MessageHandler},
    message::Message,
    update::Update,
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
        offset: Option<i32>,
        limit: Option<i32>,
        timeout: Option<i32>,
        allowed_updates: Option<Vec<String>>,
    ) -> Result<Updates, Error> {
        let update = GetUpdates {
            offset,
            limit,
            timeout,
            allowed_updates,
        };
        let mut updates = update.perform(&self.api_key).await?;

        updates.updates = updates
            .updates
            .into_iter()
            .filter(|upd| upd.update_id > self.last_update)
            .collect();

        self.last_update = updates
            .updates
            .last()
            .map(|upd| upd.update_id)
            .unwrap_or(self.last_update);
        log::info!("updated last processed update to {}", self.last_update);
        Ok(updates)
    }

    pub async fn get_all_updates(&mut self) -> Result<Updates, Error> {
        self.get_updates(None, None, None, None).await
    }

    pub async fn send_message(&self, chat_id: String, text: String) -> Result<(), Error> {
        SendMessage { chat_id, text }.perform(&self.api_key).await?;
        Ok(())
    }
}
