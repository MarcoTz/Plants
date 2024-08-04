use std::fmt;

pub enum Error {
    OtherError(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OtherError(s) => frmt.write_str(s),
        }
    }
}

impl<T: ToString> From<T> for Error {
    fn from(s: T) -> Error {
        Error::OtherError(s.to_string())
    }
}
