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

#[cfg(test)]
mod send_message_tests {
    use super::{BotMethod, SendMessage};
    use crate::test_common::{load_config, JSONData};

    fn example_message(data: &JSONData) -> SendMessage {
        SendMessage {
            chat_id: data.white_list.get(0).unwrap().to_string(),
            text: "Testing Message".to_owned(),
        }
    }

    #[test]
    fn send_url() {
        let data = load_config();
        let message = example_message(&data);
        assert_eq!(
            message.get_url(&data.api_key),
            format!("https://api.telegram.org/bot{}/sendMessage", data.api_key)
        );
    }

    #[tokio::test]
    async fn perform_send() {
        let data = load_config();
        let message = example_message(&data);
        let res = message.perform(&data.api_key).await;
        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn perform_send_wrong_key() {
        let data = load_config();
        let message = example_message(&data);
        let res = message.perform("not a valid key").await;
        assert!(res.is_err())
    }

    #[tokio::test]
    async fn perform_send_wrong_id() {
        let data = load_config();
        let message = SendMessage {
            chat_id: "not a valid chat".to_owned(),
            text: "".to_owned(),
        };
        let res = message.perform(&data.api_key).await;
        assert!(res.is_err())
    }
}
