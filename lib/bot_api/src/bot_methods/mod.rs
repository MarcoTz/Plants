use crate::errors::{Error, RequestError};
use reqwest::{Response, StatusCode};

mod get_updates;
mod send_message;

pub use get_updates::GetUpdates;
pub use send_message::SendMessage;

const API_URL: &str = "https://api.telegram.org/bot";

pub trait BotMethod {
    type Res;

    fn get_endpoint(&self) -> String;

    fn get_url(&self, api_key: &str) -> String {
        API_URL.to_owned() + api_key + "/" + &self.get_endpoint().to_string()
    }

    fn perform(
        &self,
        api_key: &str,
    ) -> impl std::future::Future<Output = Result<Self::Res, Error>> + Send;

    fn check_status(&self, resp: &Response) -> Result<(), Error> {
        if resp.status() == StatusCode::OK {
            Ok(())
        } else {
            log::warn!("Got status code {}", resp.status());
            Err(RequestError {
                url: Some(self.get_endpoint()),
                status: Some(resp.status().as_u16()),
            }
            .into())
        }
    }
}
