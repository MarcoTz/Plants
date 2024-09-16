use super::BotMethod;
use crate::{errors::Error, parse_json::get_filename};
use bytes::Bytes;
use url::Url;

pub struct DownloadImage {
    pub file_id: String,
}

impl BotMethod for DownloadImage {
    type Res = Bytes;

    fn get_endpoint(&self) -> String {
        "getFile".to_owned()
    }

    async fn perform(&self, api_key: &str) -> Result<Self::Res, Error> {
        let api_url = self.get_url(api_key);
        let request_params = vec![("file_id", self.file_id.clone())];

        let request_url = Url::parse_with_params(&api_url, &request_params).unwrap();
        let resp = reqwest::get(request_url).await?;

        self.check_status(&resp)?;

        let photo_val: serde_json::Value = resp.json().await?;
        let path = get_filename(photo_val)?;
        let download_url = format!("https://api.telegram.org/file/bot{}/{}", api_key, path);

        let dl_resp = reqwest::get(download_url).await?;
        self.check_status(&dl_resp)?;
        let resp_bytes = dl_resp.bytes().await?;
        Ok(resp_bytes)
    }
}
