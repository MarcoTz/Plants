use plants::errors as plant_err;
use std::{fmt, io::Error as IOError, path::PathBuf};

#[derive(Debug)]
pub enum ConversionType {
    Bool,
    Int,
    Str,
    Date,
}

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

pub struct IOErr {
    pub kind: String,
}
pub struct ParseError {
    pub ty: String,
    pub input: String,
}

pub struct CSVError {
    pub path: PathBuf,
    pub err_msg: String,
}

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

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IOErr(IOErr { kind }) => frmt.write_str(&format!("IO Error: {kind}")),
            Error::CSVError(CSVError {
                path: file_name,
                err_msg: msg,
            }) => frmt.write_str(&format!(
                "Could not load csv file {file_name:?}, message: {msg}"
            )),
            Error::ParseError(ParseError { ty, input }) => {
                frmt.write_str(&format!("Could not parse {ty:?}, from input \"{input}\"",))
            }
            Error::SerializeError(SerializeError { path, err_msg: msg }) => frmt.write_str(
                &format!("Could not serliaize for file {path:?}, message: {msg}"),
            ),

            Error::PlantError(err) => fmt::Debug::fmt(err, frmt),
            Error::SpeciesNotFound(name) => frmt.write_str(&format!("Species {name} not found")),
            Error::PlantNotFound(name) => frmt.write_str(&format!("Plant {name} not found")),
            Error::LocationNotFound(name) => {
                frmt.write_str(&format!("Could not find location {name}"))
            }
        }
    }
}
