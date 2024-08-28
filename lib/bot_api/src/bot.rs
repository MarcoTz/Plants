use super::{
    errors::{Error, RequestError},
    update::Updates,
};
use reqwest;
use reqwest::{StatusCode, Url};

pub struct Bot {
    pub api_key: String,
}

impl Bot {
    fn get_base_url(&self) -> String {
        format!("https://api.telegram.org/bot{}/", self.api_key)
    }
    pub async fn get_updates(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        timeout: Option<i32>,
        allowed_updates: Option<Vec<String>>,
    ) -> Result<Updates, Error> {
        let api_url = self.get_base_url() + "getUpdates";
        let mut request_params = vec![];
        if let Some(off) = offset {
            request_params.push(("offset", off.to_string()));
        }
        if let Some(lim) = limit {
            request_params.push(("limit", lim.to_string()));
        }
        if let Some(timeout) = timeout {
            request_params.push(("timeout", timeout.to_string()));
        }
        if let Some(updates) = allowed_updates {
            request_params.push((
                "allowed_updates",
                "[".to_owned() + &updates.join(", ").to_string() + "]",
            ));
        }

        log::info!("Getting Updates");
        let request_url = Url::parse_with_params(&api_url, &request_params)?;
        let resp = reqwest::get(request_url).await?;
        if resp.status() != StatusCode::OK {
            log::warn!("Could not get updates");
            Err(RequestError {
                url: Some(api_url),
                status: Some(resp.status().as_u16()),
            }
            .into())
        } else {
            log::info!("Successfully got updates");
            let updates: serde_json::Value = resp.json().await?;
            updates.try_into()
        }
    }

    pub async fn get_all_updates(&self) -> Result<Updates, Error> {
        self.get_updates(None, None, None, None).await
    }
}
