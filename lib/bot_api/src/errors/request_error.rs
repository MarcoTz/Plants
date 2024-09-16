use super::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct RequestError {
    pub url: Option<String>,
    pub status: Option<u16>,
}

impl From<RequestError> for Error {
    fn from(req_err: RequestError) -> Error {
        Error::Request(req_err)
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
impl fmt::Display for RequestError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        let url_str = match &self.url {
            None => "".to_owned(),
            Some(url) => format!(" to url {url}"),
        };
        let status_str = match self.status {
            None => "".to_owned(),
            Some(status) => format!(" with status {status}"),
        };
        let msg = format!("Request{url_str} failed{status_str}");
        frmt.write_str(&msg)
    }
}

#[cfg(test)]
mod request_err_tests {
    use super::RequestError;

    #[test]
    fn display_full_err() {
        let result = format!(
            "{}",
            RequestError {
                url: Some("no url".to_owned()),
                status: Some(400)
            }
        );
        let expected = "Request to url no url failed with status 400";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_no_url() {
        let result = format!(
            "{}",
            RequestError {
                url: None,
                status: Some(400)
            }
        );
        let expected = "Request failed with status 400";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_no_status() {
        let result = format!(
            "{}",
            RequestError {
                url: Some("no url".to_owned()),
                status: None
            }
        );
        let expected = "Request to url no url failed";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_neither() {
        let result = format!(
            "{}",
            RequestError {
                url: None,
                status: None
            }
        );
        let expected = "Request failed";
        assert_eq!(result, expected)
    }

    #[tokio::test]
    async fn display_from_reqwest() {
        let err = reqwest::get("https://doesnotexist.notadomain")
            .await
            .unwrap_err();
        let result = format!("{}", <reqwest::Error as Into<RequestError>>::into(err));
        let expected = "Request to url https://doesnotexist.notadomain/ failed";
        assert_eq!(result, expected)
    }
}
