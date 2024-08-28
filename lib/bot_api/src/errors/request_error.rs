use super::Error;
use std::fmt;

#[derive(Clone)]
pub struct RequestError {
    pub url: Option<String>,
    pub status: Option<u16>,
}

impl From<RequestError> for Error {
    fn from(req_err: RequestError) -> Error {
        Error::RequestError(req_err)
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(req_err: reqwest::Error) -> RequestError {
        RequestError {
            url: req_err.url().map(|url| url.to_string()),
            status: req_err.status().map(|st| st.as_u16()),
        }
    }
}
impl fmt::Debug for RequestError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        let url_str = match &self.url {
            None => "".to_owned(),
            Some(url) => format!("to url {url}"),
        };
        let status_str = match self.status {
            None => "".to_owned(),
            Some(status) => format!("with status {status}"),
        };
        let msg = format!("Request {url_str} failed {status_str}");
        frmt.write_str(&msg)
    }
}
