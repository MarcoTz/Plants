use std::fmt;

#[derive(Debug)]
pub enum Error {
    SQLiteErr(sqlite::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SQLiteErr(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
