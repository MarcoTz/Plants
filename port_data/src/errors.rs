use chrono::ParseError;
use database::file_backend::errors::Error as DBError;
use std::{
    fmt,
    io::Error as IOError,
    num::{ParseFloatError, ParseIntError},
    path::PathBuf,
    str::ParseBoolError,
};

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Input(String),
    DB(DBError),
    BadHealth(i32),
    PlantNotFound(String),
    FS(IOError),
    Path(String),
    FileName(String),
    MissingFiles(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parse(msg) => frmt.write_str(&format!("Could not parse {msg}")),
            Error::Input(msg) => frmt.write_str(&format!("Could not read input {msg}")),
            Error::DB(err) => frmt.write_str(&format!("{err}")),
            Error::BadHealth(i) => frmt.write_str(&format!("{i} is not a valid value for health")),
            Error::PlantNotFound(name) => frmt.write_str(&format!("Plant {name} was not found")),
            Error::FS(ioerr) => ioerr.fmt(frmt),
            Error::Path(msg) => frmt.write_str(&format!("Could not get file name for {msg}")),
            Error::FileName(msg) => frmt.write_str(&format!("Bad filename format {msg}")),
            Error::MissingFiles(files) => frmt.write_str(&format!("Files {files:?} do not exist")),
        }
    }
}

impl std::error::Error for Error {}

impl From<IOError> for Error {
    fn from(io_err: IOError) -> Error {
        Error::FS(io_err)
    }
}

impl From<ParseBoolError> for Error {
    fn from(_: ParseBoolError) -> Error {
        Error::Parse("bool".to_owned())
    }
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Error {
        Error::Parse("date".to_owned())
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Error {
        Error::Parse("float".to_owned())
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::Parse("int".to_owned())
    }
}

impl From<DBError> for Error {
    fn from(err: DBError) -> Error {
        Error::DB(err)
    }
}
