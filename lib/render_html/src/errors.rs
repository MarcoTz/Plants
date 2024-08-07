use std::fmt;

pub enum Error {
    PageError(pages::errors::Error),
    DatabaseError(database::errors::Error),
    IOError(std::io::Error),
}

impl From<pages::errors::Error> for Error {
    fn from(page_err: pages::errors::Error) -> Error {
        Error::PageError(page_err)
    }
}

impl From<database::errors::Error> for Error {
    fn from(db_err: database::errors::Error) -> Error {
        Error::DatabaseError(db_err)
    }
}
impl From<std::io::Error> for Error {
    fn from(io_err: std::io::Error) -> Error {
        Error::IOError(io_err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PageError(page_err) => page_err.fmt(frmt),
            Error::DatabaseError(db_err) => db_err.fmt(frmt),
            Error::IOError(io_err) => io_err.fmt(frmt),
        }
    }
}
