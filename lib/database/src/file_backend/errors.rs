use plants::errors as plant_err;
use std::{fmt, io::Error as IOError, path::PathBuf};

#[derive(Debug)]
pub enum ConversionType {
    Bool,
    Int,
    Str,
    Date,
}

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    IOErr(IOErr),
    CSVError(CSVError),
    SerializeError(SerializeError),
    PlantError(plant_err::Error),
    SpeciesNotFound(String),
    LocationNotFound(String),
    PlantNotFound(String),
}
impl std::error::Error for Error {}

#[derive(Debug)]
pub struct IOErr {
    pub kind: String,
}

#[derive(Debug)]
pub struct ParseError {
    pub ty: String,
    pub input: String,
}

#[derive(Debug)]
pub struct CSVError {
    pub path: PathBuf,
    pub err_msg: String,
}

#[derive(Debug)]
pub struct SerializeError {
    pub path: PathBuf,
    pub err_msg: String,
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        IOErr {
            kind: err.kind().to_string(),
        }
        .into()
    }
}

impl From<IOErr> for Error {
    fn from(err: IOErr) -> Error {
        Error::IOErr(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::ParseError(err)
    }
}

impl From<CSVError> for Error {
    fn from(err: CSVError) -> Error {
        Error::CSVError(err)
    }
}

impl From<SerializeError> for Error {
    fn from(err: SerializeError) -> Error {
        Error::SerializeError(err)
    }
}

impl From<plant_err::Error> for Error {
    fn from(err: plant_err::Error) -> Error {
        Error::PlantError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IOErr(io_err) => io_err.fmt(frmt),
            Error::CSVError(csv_err) => csv_err.fmt(frmt),
            Error::ParseError(parse_err) => parse_err.fmt(frmt),
            Error::SerializeError(serialize_err) => serialize_err.fmt(frmt),
            Error::PlantError(err) => err.fmt(frmt),
            Error::SpeciesNotFound(name) => frmt.write_str(&format!("Species {name} not found")),
            Error::PlantNotFound(name) => frmt.write_str(&format!("Plant {name} not found")),
            Error::LocationNotFound(name) => {
                frmt.write_str(&format!("Could not find location {name}"))
            }
        }
    }
}

impl fmt::Display for IOErr {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        frmt.write_str(&format!("IO Error: {}", self.kind))
    }
}
impl fmt::Display for CSVError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        frmt.write_str(&format!(
            "Could not load csv file {:?}, message: {}",
            self.path, self.err_msg
        ))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        frmt.write_str(&format!(
            "Could not parse {:?}, from input \"{}\"",
            self.ty, self.input
        ))
    }
}

impl fmt::Display for SerializeError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        frmt.write_str(&format!(
            "Could not serliaize for file {:?}, message: {}",
            self.path, self.err_msg
        ))
    }
}
#[cfg(test)]
mod error_tests {
    use super::{CSVError, Error, IOErr, ParseError, SerializeError};
    use std::{fs::File, path::PathBuf};

    fn example_parse_err() -> ParseError {
        ParseError {
            ty: "Int".to_owned(),
            input: "true".to_owned(),
        }
    }

    fn example_io_err() -> IOErr {
        IOErr {
            kind: "test".to_owned(),
        }
    }

    fn example_csv_err() -> CSVError {
        CSVError {
            path: PathBuf::from("./"),
            err_msg: "testing".to_owned(),
        }
    }

    fn example_serialize_err() -> SerializeError {
        SerializeError {
            path: PathBuf::from("./"),
            err_msg: "testing".to_owned(),
        }
    }

    #[test]
    fn from_io_err() {
        let io_err = File::open("../../testing/doesnotexist").unwrap_err();
        let result = format!("{}", <std::io::Error as Into<Error>>::into(io_err));
        let expected = "IO Error: entity not found";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_ioerr_into() {
        let io_err = example_io_err();
        let expected = format!("{}", io_err);
        let result = format!("{}", <IOErr as Into<Error>>::into(io_err));
        assert_eq!(result, expected)
    }

    #[test]
    fn display_csverr_into() {
        let csv_err = example_csv_err();
        let expected = format!("{}", csv_err);
        let result = format!("{}", <CSVError as Into<Error>>::into(csv_err));
        assert_eq!(result, expected)
    }

    #[test]
    fn display_parseerr_into() {
        let parse_err = example_parse_err();
        let expected = format!("{}", parse_err);
        let result = format!("{}", <ParseError as Into<Error>>::into(parse_err));
        assert_eq!(result, expected)
    }

    #[test]
    fn display_serializeerr_into() {
        let serialize_err = example_serialize_err();
        let expected = format!("{}", serialize_err);
        let result = format!("{}", <SerializeError as Into<Error>>::into(serialize_err));
        assert_eq!(result, expected)
    }

    #[test]
    fn display_parse_err() {
        let result = format!("{}", example_parse_err());
        let expected = "Could not parse \"Int\", from input \"true\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn displa_io_err() {
        let result = format!("{}", example_io_err());
        let expected = "IO Error: test";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_csv_err() {
        let result = format!("{}", example_csv_err());
        let expected = "Could not load csv file \"./\", message: testing";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_serialice_err() {
        let result = format!("{}", example_serialize_err());
        let expected = "Could not serliaize for file \"./\", message: testing";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_plant_err() {
        let plant_err = plants::errors::Error::SunlightError("not sunlight".to_owned());
        let expected = format!("{}", plant_err);
        let result = format!(
            "{}",
            <plants::errors::Error as Into<Error>>::into(plant_err)
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn display_species_not_found() {
        let result = format!("{}", Error::SpeciesNotFound("not a species".to_owned()));
        let expected = "Species not a species not found";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_location_not_found() {
        let result = format!("{}", Error::LocationNotFound("not a location".to_owned()));
        let expected = "Could not find location not a location";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_plant_not_found() {
        let result = format!("{}", Error::PlantNotFound("not a plant".to_owned()));
        let expected = "Plant not a plant not found";
        assert_eq!(result, expected)
    }
}
