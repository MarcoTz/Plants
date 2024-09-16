use super::{
    bot_methods::{BotMethod, DownloadImage, GetUpdates, SendMessage},
    errors::Error,
    update::Updates,
};
use bytes::Bytes;

#[derive(Debug, PartialEq, Eq)]
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
}

#[cfg(test)]
mod bot_tests {
    use super::Bot;
    use crate::test_common::load_config;

    #[test]
    fn new_bot() {
        let data = load_config();
        let result = Bot::new(data.api_key.clone());
        assert_eq!(
            result,
            Bot {
                api_key: data.api_key,
                last_update: 0
            }
        )
    }

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
