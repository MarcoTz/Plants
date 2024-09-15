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
    MissingImage,
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
            Error::MissingImage => write!(frmt, "No image sizes were provided"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(req_err: reqwest::Error) -> Error {
        Error::Request(req_err.into())
    }
}

impl From<ParseError> for Error {
    fn from(parse_err: ParseError) -> Error {
        Error::Parse(parse_err)
    }
}

impl From<url::ParseError> for Error {
    fn from(parse_err: url::ParseError) -> Error {
        Error::Parse(parse_err.into())
    }
}

impl From<SerializeError> for Error {
    fn from(serialize_err: SerializeError) -> Error {
        Error::Serialize(serialize_err)
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

#[cfg(test)]
mod error_tests {
    use super::{BadResponse, Error, ParseError, RequestError, SerializeError};
    use crate::{chat::Chat, message::Message, update::Update};

    #[test]
    fn display_request() {
        let request_err = RequestError {
            url: Some("google.com".to_owned()),
            status: Some(404),
        };

        let expected = format!("{}", request_err);
        let result = format!("{}", <RequestError as Into<Error>>::into(request_err));
        assert_eq!(result, expected)
    }

    #[test]
    fn display_parse() {
        let parse_err = ParseError {
            ty: "Integer".to_owned(),
        };
        let expected = format!("{}", parse_err);
        let result = format!("{}", <ParseError as Into<Error>>::into(parse_err));
        assert_eq!(result, expected)
    }

    #[test]
    fn display_serialize() {
        let serialize_err = SerializeError {
            line: 1,
            column: 1,
            kind: "testing error".to_owned(),
            reason: "testing".to_owned(),
        };
        let expected = format!("{}", serialize_err);
        let result = format!("{}", <SerializeError as Into<Error>>::into(serialize_err));
        assert_eq!(result, expected)
    }

    #[test]
    fn display_bad_response() {
        let resp_error = BadResponse::MissingField("fieldname".to_owned());
        let expected = format!("{}", resp_error);
        let result = format!("{}", <BadResponse as Into<Error>>::into(resp_error));
        assert_eq!(result, expected)
    }

    fn example_message() -> Message {
        Message {
            id: 1,
            date: 1,
            from: None,
            chat: Chat {
                id: 2,
                ty: "message".to_owned(),
                title: None,
                username: None,
                first_name: None,
                last_name: None,
            },
            text: None,
            caption: None,
            photo: None,
            entities: None,
        }
    }
    #[test]
    fn display_is_command() {
        let result = format!("{}", Error::MessageIsCommand(Box::new(example_message())));
        let expected ="Message Message { id: 1, date: 1, from: None, chat: Chat { id: 2, ty: \"message\", title: None, username: None, first_name: None, last_name: None }, text: None, photo: None, entities: None } is a command and cannot be handled with message handler";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_is_message() {
        let result = format!("{}", Error::CommandIsMessage(Box::new(example_message())));
        let expected = "Message Message { id: 1, date: 1, from: None, chat: Chat { id: 2, ty: \"message\", title: None, username: None, first_name: None, last_name: None }, text: None, photo: None, entities: None } is message and cannot be handled with command handler";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_empty_message() {
        let result = format!("{}", Error::EmptyMessage(Box::new(example_message())));
        let expected = "Message Message { id: 1, date: 1, from: None, chat: Chat { id: 2, ty: \"message\", title: None, username: None, first_name: None, last_name: None }, text: None, photo: None, entities: None } is empty";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_no_message() {
        let result = format!(
            "{}",
            Error::NoMessage(Box::new(Update {
                update_id: 1,
                content: None
            }))
        );
        let expected = "No message for update Update { update_id: 1, content: None }";
        assert_eq!(result, expected)
    }

    #[tokio::test]
    async fn display_reqwest_err() {
        let err =
            <reqwest::Error as Into<Error>>::into(reqwest::get("not a url").await.unwrap_err());
        let result = format!("{}", err);
        let expected = "Request failed";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_url_err() {
        let err =
            <url::ParseError as Into<Error>>::into(url::Url::parse("not a valid url").unwrap_err());
        let result = format!("{}", err);
        let expected = "Could not parse relative URL without a base";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_serde_err() {
        let err = <serde_json::Error as Into<Error>>::into(
            serde_json::from_str::<serde_json::Value>("{bad json}").unwrap_err(),
        );
        let result = format!("{}", err);
        let expected =
            "Could not deserialize\n Error io kind: None, Category: Syntax at line 1, column 2";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_missing_img() {
        let result = format!("{}", Error::MissingImage);
        let expected = "No image sizes were provided";
        assert_eq!(result, expected)
    }
}
