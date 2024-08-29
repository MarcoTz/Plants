mod bad_response;
mod parse_error;
mod request_error;
mod serialize_error;

pub use bad_response::{BadResponse, BadValue, WrongType};
pub use parse_error::ParseError;
pub use request_error::RequestError;
pub use serialize_error::SerializeError;

use super::{message::Message, update::Update};
use std::fmt;

#[derive(Clone, Debug)]
pub enum Error {
    Request(RequestError),
    Parse(ParseError),
    Serialize(SerializeError),
    BadResponse(BadResponse),
    MessageIsCommand(Box<Message>),
    CommandIsMessage(Box<Message>),
    EmptyMessage(Box<Message>),
    NoMessage(Box<Update>),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Request(req_err) => req_err.fmt(frmt),
            Error::Parse(parse_err) => parse_err.fmt(frmt),
            Error::Serialize(ser_err) => ser_err.fmt(frmt),
            Error::BadResponse(bad_resp) => bad_resp.fmt(frmt),
            Error::MessageIsCommand(msg) => frmt.write_str(&format!(
                "Message {msg:?} is a command and cannot be handled with message handler"
            )),
            Error::CommandIsMessage(msg) => frmt.write_str(&format!(
                "Message {msg:?} is message and cannot be handled with command handler"
            )),
            Error::EmptyMessage(msg) => frmt.write_str(&format!("Message {msg:?} is empty")),
            Error::NoMessage(update) => {
                frmt.write_str(&format!("No message for update {update:?}"))
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(req_err: reqwest::Error) -> Error {
        Error::Request(req_err.into())
    }
}

impl From<url::ParseError> for Error {
    fn from(parse_err: url::ParseError) -> Error {
        Error::Parse(parse_err.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(serde_err: serde_json::Error) -> Error {
        Error::Serialize(serde_err.into())
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
