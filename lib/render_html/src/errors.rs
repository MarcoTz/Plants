use std::fmt;

#[derive(Debug)]
pub enum Error {
    PageError(pages::errors::Error),
    IOError(std::io::Error),
    Other(Box<dyn std::error::Error>),
}

impl From<pages::errors::Error> for Error {
    fn from(page_err: pages::errors::Error) -> Error {
        Error::PageError(page_err)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_err: std::io::Error) -> Error {
        Error::IOError(io_err)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Error {
        Error::Other(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PageError(page_err) => page_err.fmt(frmt),
            Error::IOError(io_err) => io_err.fmt(frmt),
            Error::Other(err) => err.fmt(frmt),
        }
    }
}

impl std::error::Error for Error {}
