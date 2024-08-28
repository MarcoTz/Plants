mod bad_response;
mod parse_error;
mod request_error;
mod serialize_error;

pub use bad_response::{BadResponse, BadValue, WrongType};
pub use parse_error::ParseError;
pub use request_error::RequestError;
pub use serialize_error::SerializeError;

use std::fmt;

#[derive(Clone)]
pub enum Error {
    RequestError(RequestError),
    ParseError(ParseError),
    SerializeError(SerializeError),
    BadResponse(BadResponse),
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RequestError(req_err) => req_err.fmt(frmt),
            Error::ParseError(parse_err) => parse_err.fmt(frmt),
            Error::SerializeError(ser_err) => ser_err.fmt(frmt),
            Error::BadResponse(bad_resp) => bad_resp.fmt(frmt),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(req_err: reqwest::Error) -> Error {
        Error::RequestError(req_err.into())
    }
}

impl From<url::ParseError> for Error {
    fn from(parse_err: url::ParseError) -> Error {
        Error::ParseError(parse_err.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(serde_err: serde_json::Error) -> Error {
        Error::SerializeError(serde_err.into())
    }
}

impl From<BadValue> for Error {
    fn from(bad_val: BadValue) -> Error {
        Error::BadResponse(bad_val.into())
    }
}

impl From<WrongType> for Error {
    fn from(wrong_ty: WrongType) -> Error {
        Error::BadResponse(wrong_ty.into())
    }
}
