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
    Logger(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parse(msg) => write!(f, "Could not parse {msg}"),
            Error::Input(msg) => write!(f, "Could not read input {msg}"),
            Error::DB(err) => write!(f, "{err}"),
            Error::BadHealth(i) => write!(f, "{i} is not a valid value for health"),
            Error::PlantNotFound(name) => write!(f, "Plant {name} not found."),
            Error::FS(ioerr) => ioerr.fmt(f),
            Error::Path(msg) => write!(f, "Could not get filename for {msg}."),
            Error::FileName(msg) => write!(f, "Bad filename format: {msg}."),
            Error::MissingFiles(files) => write!(f, "Files {files:?} do not exist."),
            Error::Logger(msg) => write!(f, "Logger could not be initialized: {msg}."),
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

#[cfg(test)]
mod error_tests {
    use super::Error;
    use chrono::{NaiveDate, ParseError};
    use database::file_backend::errors::Error as DBError;
    use std::{
        io::Error as IOError,
        num::{ParseFloatError, ParseIntError},
        path::PathBuf,
        str::ParseBoolError,
    };

    #[test]
    fn display_parse() {
        let result = format!("{}", Error::Parse("parsing".to_owned()));
        let expected = "Could not parse parsing";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_input() {
        let result = format!("{}", Error::Input("input".to_owned()));
        let expected = "Could not read input input";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_db() {
        let result = format!("{}", Error::DB(DBError::PlantNotFound("plant".to_owned())));
        let expected = "Plant plant not found";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_bad_health() {
        let result = format!("{}", Error::BadHealth(6));
        let expected = "6 is not a valid value for health";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_plant_not_found() {
        let result = format!("{}", Error::PlantNotFound("plant".to_owned()));
        let expected = "Plant plant not found.";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_io() {
        let result = format!(
            "{}",
            Error::FS(IOError::new(
                std::io::ErrorKind::Other,
                "error message".to_owned()
            ))
        );
        let expected = "error message";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_path() {
        let result = format!("{}", Error::Path("path".to_owned()));
        let expected = "Could not get filename for path.";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_filename() {
        let result = format!("{}", Error::FileName("badname".to_owned()));
        let expected = "Bad filename format: badname.";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_missing_files() {
        let result = format!("{}", Error::MissingFiles(PathBuf::from("file1, file2")));
        let expected = "Files \"file1, file2\" do not exist.";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_logger() {
        let result = format!("{}", Error::Logger("unknown error".to_owned()));
        let expected = "Logger could not be initialized: unknown error.";
        assert_eq!(result, expected)
    }

    #[test]
    fn io_into() {
        let result = format!(
            "{}",
            <IOError as Into<Error>>::into(IOError::new(
                std::io::ErrorKind::Other,
                "error message".to_owned()
            ))
        );
        let expected = format!(
            "{}",
            IOError::new(std::io::ErrorKind::Other, "error message".to_owned())
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_bool_into() {
        let result = format!(
            "{}",
            <ParseBoolError as Into<Error>>::into("notbool".parse::<bool>().unwrap_err())
        );
        let expected = format!("{}", Error::Parse("bool".to_owned()));
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_date_into() {
        let result = format!(
            "{}",
            <ParseError as Into<Error>>::into(
                NaiveDate::parse_from_str("not a date", "%d.%m.%Y").unwrap_err()
            )
        );
        let expected = format!("{}", Error::Parse("date".to_owned()));
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_float_into() {
        let result = format!(
            "{}",
            <ParseFloatError as Into<Error>>::into("not a float".parse::<f32>().unwrap_err())
        );
        let expected = format!("{}", Error::Parse("float".to_owned()));
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_int_into() {
        let result = format!(
            "{}",
            <ParseIntError as Into<Error>>::into("not an int".parse::<i32>().unwrap_err())
        );
        let expected = format!("{}", Error::Parse("int".to_owned()));
        assert_eq!(result, expected)
    }

    #[test]
    fn db_into() {
        let result = format!(
            "{}",
            <DBError as Into<Error>>::into(DBError::PlantNotFound("plant".to_owned()))
        );
        let expected = format!("{}", DBError::PlantNotFound("plant".to_owned()));
        assert_eq!(result, expected)
    }
}
