use std::fmt;

#[derive(Debug)]
pub enum Error {
    SQLiteErr(sqlite::Error),
    UnexpectedColumn { name: String },
    LocationNotFound { name: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SQLiteErr(e) => e.fmt(f),
            Error::UnexpectedColumn { name } => write!(f, "Unexpected column {name} in sql result"),
            Error::LocationNotFound { name } => write!(f, "Could not find location {name}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<sqlite::Error> for Error {
    fn from(err: sqlite::Error) -> Error {
        Error::SQLiteErr(err)
    }
}
