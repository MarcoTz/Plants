use super::{
    bot_methods::{BotMethod, DownloadImage, GetUpdates, SendMessage},
    commands::Command,
    errors::Error,
    handlers::Handler,
    update::Update,
    update::Updates,
};
use bytes::Bytes;

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

    pub async fn download_image(&self, file_id: String) -> Result<Bytes, Error> {
        DownloadImage { file_id }.perform(&self.api_key).await
    }

    pub async fn handle_update<U: Handler<T>, T: Command>(
        &mut self,
        update: Update,
        handler: &mut U,
    ) -> Result<(), Error> {
        self.last_update = update.update_id;
        let msg = update.get_message()?;
        if msg.is_command() {
            let cmd: T = msg.get_command().map_err(Error::Other)?;
            handler.handle_cmd(self, cmd, msg).await;
        } else if let Some(photo) = msg.photo.clone() {
            handler.handle_img(self, photo, msg).await;
        } else {
            handler.handle_msg(self, msg).await;
        }
        Ok(())
    }

    pub async fn handle_updates<U: Handler<T>, T: Command>(
        &mut self,
        handler: &mut U,
    ) -> Result<(), Error> {
        let updates = self.get_all_updates().await?;
        for update in updates.updates {
            self.handle_update(update, handler).await?;
        }
        Ok(())
    }

    pub async fn run<U: Handler<T>, T: Command>(&mut self, handler: &mut U) {
        loop {
            match self.handle_updates(handler).await {
                Ok(_) => (),
                Err(err) => {
                    log::error!("Bot encountered an error: {err}");
                }
            }
        }
    }
}

#[cfg(test)]
mod bot_tests {
    use super::Bot;
    use crate::test_common::load_config;

    #[tokio::test]
    async fn bot_updates() {
        let data = load_config();
        Bot::new(data.api_key)
            .get_updates(None, None, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn update_fail() {
        let result = Bot::new("".to_owned()).get_updates(None, None, None).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn all_updates() {
        let data = load_config();
        let mut bot = Bot::new(data.api_key);
        bot.last_update = 4;
        let result = bot.get_all_updates().await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn all_updates_fail() {
        let result = Bot::new("".to_owned()).get_all_updates().await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn send_message() {
        let data = load_config();
        let bot = Bot::new(data.api_key);
        let chat_id = data.white_list.get(0).unwrap();
        let res = bot
            .send_message(chat_id.to_string(), "Running Tests".to_owned())
            .await;
        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn message_fail() {
        let data = load_config();
        let bot = Bot::new(data.api_key);
        let res = bot
            .send_message("not a real chat".to_owned(), "Running Tests".to_owned())
            .await;
        assert!(res.is_err())
    }
}
