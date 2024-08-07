use super::file_backend;
use std::fmt;

pub enum Error {
    FileDBError(file_backend::errors::Error),
}

impl From<file_backend::errors::Error> for Error {
    fn from(file_err: file_backend::errors::Error) -> Error {
        Error::FileDBError(file_err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FileDBError(file_err) => file_err.fmt(frmt),
        }
    }
}
