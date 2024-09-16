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

#[cfg(test)]
mod error_tests {
    use super::Error;
    use pages::errors::Error as PageError;
    use std::io::Error as IOError;

    fn example_page_err() -> PageError {
        PageError::LocationError(vec!["location1".to_owned(), "location2".to_owned()])
    }
    fn example_io_err() -> IOError {
        IOError::new(std::io::ErrorKind::Other, "an error")
    }

    #[test]
    fn page_err_into() {
        let result = format!("{}", <PageError as Into<Error>>::into(example_page_err()));
        let expected = format!("{}", Error::PageError(example_page_err()));
        assert_eq!(result, expected)
    }

    #[test]
    fn io_err_into() {
        let result = format!("{}", <IOError as Into<Error>>::into(example_io_err()));
        let expected = format!("{}", example_io_err());
        assert_eq!(result, expected)
    }

    #[test]
    fn dyn_err_into() {
        let result = format!(
            "{}",
            <Box<dyn std::error::Error> as Into<Error>>::into(Box::new(example_io_err()))
        );
        let expected = format!("{}", example_io_err());
        assert_eq!(result, expected)
    }
}
