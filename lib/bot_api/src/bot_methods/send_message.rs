use super::BotMethod;
use crate::{errors::Error, message::Message, parse_json::check_ok};
use reqwest::Client;
use serde_json::Value;

pub struct SendMessage {
    pub chat_id: String,
    pub text: String,
}

impl BotMethod for SendMessage {
    type Res = Message;

    fn get_endpoint(&self) -> String {
        "sendMessage".to_owned()
    }

    async fn perform(&self, api_key: &str) -> Result<Self::Res, Error> {
        let client = Client::new();
        let url = self.get_url(api_key);
        let params = [("chat_id", &self.chat_id), ("text", &self.text)];
        let resp = client.post(url).form(&params).send().await?;
        self.check_status(&resp)?;
        let resp_json: Value = resp.json().await?;
        let resp_ok = check_ok(resp_json)?;
        resp_ok.try_into()
    }
}
