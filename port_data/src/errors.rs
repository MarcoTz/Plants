use chrono::ParseError;
use database::file_backend::errors::Error as DBError;
use std::{fmt, str::ParseBoolError};

pub enum Error {
    ParseError(String),
    DBError(DBError),
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError(msg) => frmt.write_str(&format!("Could not parse {msg}")),
            Error::DBError(err) => err.fmt(frmt),
        }
    }
}

impl From<ParseBoolError> for Error {
    fn from(_: ParseBoolError) -> Error {
        Error::ParseError("bool".to_owned())
    }
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Error {
        Error::ParseError("date".to_owned())
    }
}

impl From<DBError> for Error {
    fn from(err: DBError) -> Error {
        Error::DBError(err)
    }
}
