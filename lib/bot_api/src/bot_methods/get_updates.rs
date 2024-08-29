use super::BotMethod;
use crate::{errors::Error, update::Updates};
use url::Url;

pub struct GetUpdates {
    pub offset: Option<i32>,
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
        log::info!("Getting Updates");

        let request_url = Url::parse_with_params(&api_url, &request_params)?;
        let resp = reqwest::get(request_url).await?;

        self.check_status(&resp)?;

        let update_vals: serde_json::Value = resp.json().await?;
        let updates: Updates = update_vals.try_into()?;
        log::info!("Successfully got {} updates", updates.updates.len());
        Ok(updates)
    }
}
