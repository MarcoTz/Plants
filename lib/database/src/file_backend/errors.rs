use chrono;
use plants::errors as plant_err;
use std::fmt;
use std::io;
use std::num;
use std::str;

#[derive(Debug)]
pub enum ConversionType {
    Bool,
    Int,
    Str,
    Date,
}

pub struct ConversionError {
    pub from_ty: ConversionType,
    pub to_ty: ConversionType,
    pub msg: String,
}
pub enum Error {
    FilesError(io::Error),
    JSONError(serde_json::Error),
    PathError(String),
    ConversionError(ConversionError),
    PlantErr(plant_err::PlantError),
    OtherErr(String),
}

impl Into<Error> for ConversionError {
    fn into(self) -> Error {
        Error::ConversionError(self)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::FilesError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JSONError(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        ConversionError {
            from_ty: ConversionType::Str,
            to_ty: ConversionType::Int,
            msg: err.to_string(),
        }
        .into()
    }
}

impl From<plant_err::PlantError> for Error {
    fn from(err: plant_err::PlantError) -> Error {
        Error::PlantErr(err)
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Error {
        ConversionError {
            from_ty: ConversionType::Str,
            to_ty: ConversionType::Int,
            msg: err.to_string(),
        }
        .into()
    }
}

impl From<str::ParseBoolError> for Error {
    fn from(err: str::ParseBoolError) -> Error {
        ConversionError {
            from_ty: ConversionType::Str,
            to_ty: ConversionType::Bool,
            msg: err.to_string(),
        }
        .into()
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Error {
        ConversionError {
            from_ty: ConversionType::Str,
            to_ty: ConversionType::Date,
            msg: err.to_string(),
        }
        .into()
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error::OtherErr(msg)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FilesError(err) => fmt::Debug::fmt(err, frmt),
            Error::JSONError(err) => fmt::Debug::fmt(err, frmt),
            Error::PathError(err) => frmt.write_str(err),
            Error::ConversionError(err) => frmt.write_str(&format!(
                "Could not convert from {:?} to {:?}, message: {}",
                err.from_ty, err.to_ty, err.msg,
            )),
            Error::PlantErr(err) => fmt::Debug::fmt(err, frmt),
            Error::OtherErr(msg) => frmt.write_str(msg),
        }
    }
}
