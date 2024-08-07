use std::fmt;

pub enum Error {
    PageError(pages::errors::Error),
}

impl From<pages::errors::Error> for Error {
    fn from(page_err: pages::errors::Error) -> Error {
        Error::PageError(page_err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PageError(page_err) => page_err.fmt(frmt),
        }
    }
}
