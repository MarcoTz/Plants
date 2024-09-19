use super::BotMethod;
use crate::{errors::Error, update::Updates};
use url::Url;

pub struct GetUpdates {
    pub offset: Option<i64>,
    pub limit: Option<i32>,
    pub timeout: Option<i32>,
    pub allowed_updates: Option<Vec<String>>,
}

impl GetUpdates {
    fn to_get_params(&self) -> Vec<(String, String)> {
        let mut args = vec![];
        if let Some(off) = self.offset {
            args.push(("offset".to_owned(), off.to_string()));
        }
        if let Some(lim) = self.limit {
            args.push(("limit".to_owned(), lim.to_string()));
        }

        if let Some(time) = self.timeout {
            args.push(("timeout".to_owned(), time.to_string()));
        }

        if let Some(updates) = self.allowed_updates.clone() {
            args.push((
                "allowed_updates".to_owned(),
                "[".to_owned() + &updates.join(",") + "]",
            ));
        }

        args
    }
}

impl BotMethod for GetUpdates {
    type Res = Updates;

    fn get_endpoint(&self) -> String {
        "getUpdates".to_owned()
    }

    async fn perform(&self, api_key: &str) -> Result<Updates, Error> {
        let api_url = self.get_url(api_key);
        let request_params = self.to_get_params();

        let request_url = Url::parse_with_params(&api_url, &request_params).unwrap();
        let resp = reqwest::get(request_url).await?;

        self.check_status(&resp)?;

        let update_vals: serde_json::Value = resp.json().await?;
        let updates: Updates = update_vals.try_into()?;
        Ok(updates)
    }
}

#[cfg(test)]
mod get_updates_tests {
    use super::{BotMethod, GetUpdates};
    use crate::test_common::load_config;

    fn example_get_updates1() -> GetUpdates {
        GetUpdates {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }
    fn example_get_updates2() -> GetUpdates {
        GetUpdates {
            offset: Some(3),
            limit: Some(1),
            timeout: Some(1),
            allowed_updates: Some(vec!["message".to_owned()]),
        }
    }

    #[test]
    fn to_get_params1() {
        let result = example_get_updates1().to_get_params();
        let expected = vec![];
        assert_eq!(result, expected)
    }

    #[test]
    fn to_get_params2() {
        let result = example_get_updates2().to_get_params();
        let expected = vec![
            ("offset".to_owned(), "3".to_owned()),
            ("limit".to_owned(), "1".to_owned()),
            ("timeout".to_owned(), "1".to_owned()),
            ("allowed_updates".to_owned(), "[message]".to_owned()),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn get_url() {
        let data = load_config();
        let result = example_get_updates1().get_url(&data.api_key);
        assert_eq!(
            result,
            format!("https://api.telegram.org/bot{}/getUpdates", data.api_key)
        );
    }

    #[tokio::test]
    async fn perform_updates1() {
        let data = load_config();
        let res = example_get_updates1().perform(&data.api_key).await;
        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn perform_updates_wrong_key() {
        let res = example_get_updates1().perform("not a valid key").await;
        assert!(res.is_err())
    }
}
