use chrono::ParseError;
use database::file_backend::errors::Error as DBError;
use std::{
    fmt,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

pub enum Error {
    ParseError(String),
    InputErr(String),
    DBError(DBError),
    BadHealth(i32),
    PlantNotFound(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError(msg) => frmt.write_str(&format!("Could not parse {msg}")),
            Error::InputErr(msg) => frmt.write_str(&format!("Could not read input {msg}")),
            Error::DBError(err) => err.fmt(frmt),
            Error::BadHealth(i) => frmt.write_str(&format!("{i} is not a valid value for health")),
            Error::PlantNotFound(name) => frmt.write_str(&format!("Plant {name} was not found")),
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

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Error {
        Error::ParseError("float".to_owned())
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::ParseError("int".to_owned())
    }
}

impl From<DBError> for Error {
    fn from(err: DBError) -> Error {
        Error::DBError(err)
    }
}
