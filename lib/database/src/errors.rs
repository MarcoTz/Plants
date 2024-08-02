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
pub enum DBError {
    FilesError(io::Error),
    JSONError(serde_json::Error),
    PathError(String),
    ConversionError(ConversionError),
    PlantErr(plant_err::PlantError),
}

impl Into<DBError> for ConversionError {
    fn into(self) -> DBError {
        DBError::ConversionError(self)
    }
}

impl From<io::Error> for DBError {
    fn from(err: io::Error) -> DBError {
        DBError::FilesError(err)
    }
}

impl From<serde_json::Error> for DBError {
    fn from(err: serde_json::Error) -> DBError {
        DBError::JSONError(err)
    }
}

impl From<num::ParseIntError> for DBError {
    fn from(err: num::ParseIntError) -> DBError {
        ConversionError {
            from_ty: ConversionType::Str,
            to_ty: ConversionType::Int,
            msg: err.to_string(),
        }
        .into()
    }
}

impl From<plant_err::PlantError> for DBError {
    fn from(err: plant_err::PlantError) -> DBError {
        DBError::PlantErr(err)
    }
}

impl From<num::ParseFloatError> for DBError {
    fn from(err: num::ParseFloatError) -> DBError {
        ConversionError {
            from_ty: ConversionType::Str,
            to_ty: ConversionType::Int,
            msg: err.to_string(),
        }
        .into()
    }
}

impl From<str::ParseBoolError> for DBError {
    fn from(err: str::ParseBoolError) -> DBError {
        ConversionError {
            from_ty: ConversionType::Str,
            to_ty: ConversionType::Bool,
            msg: err.to_string(),
        }
        .into()
    }
}

impl From<chrono::ParseError> for DBError {
    fn from(err: chrono::ParseError) -> DBError {
        ConversionError {
            from_ty: ConversionType::Str,
            to_ty: ConversionType::Date,
            msg: err.to_string(),
        }
        .into()
    }
}

impl fmt::Debug for DBError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DBError::FilesError(err) => fmt::Debug::fmt(err, frmt),
            DBError::JSONError(err) => fmt::Debug::fmt(err, frmt),
            DBError::PathError(err) => frmt.write_str(err),
            DBError::ConversionError(err) => frmt.write_str(&format!(
                "Could not convert from {:?} to {:?}, message: {}",
                err.from_ty, err.to_ty, err.msg,
            )),
            DBError::PlantErr(err) => fmt::Debug::fmt(err, frmt),
        }
    }
}
