use chrono;
use plants::errors as plant_err;
use std::fmt;
use std::num;
use std::str;

#[derive(Debug)]
pub enum ConversionType {
    Bool,
    Int,
    Str,
    Date,
}

pub enum AccessType {
    Write,
    Read,
}

pub enum Error {
    ConversionError(ConversionError),
    CSVError(CSVError),
    SerializeError(SerializeError),
    FSError(FSError),
    PlantError(plant_err::Error),
}

pub struct ConversionError {
    pub from_ty: ConversionType,
    pub to_ty: ConversionType,
    pub msg: String,
}

pub struct CSVError {
    pub csv_file: String,
    pub err_msg: String,
}

pub struct SerializeError {
    pub out_path: String,
    pub err_msg: String,
    pub access: AccessType,
}

pub struct FSError {
    pub file_name: String,
    pub err_msg: String,
    pub access: AccessType,
}

impl From<ConversionError> for Error {
    fn from(err: ConversionError) -> Error {
        Error::ConversionError(err)
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

impl From<FSError> for Error {
    fn from(err: FSError) -> Error {
        Error::FSError(err)
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

impl From<plant_err::Error> for Error {
    fn from(err: plant_err::Error) -> Error {
        Error::PlantError(err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CSVError(CSVError {
                csv_file: file_name,
                err_msg: msg,
            }) => frmt.write_str(&format!(
                "Could not load csv file {file_name:?}, message: {msg}"
            )),
            Error::ConversionError(ConversionError {
                from_ty: frty,
                to_ty: toty,
                msg: err,
            }) => frmt.write_str(&format!(
                "Could not convert from {frty:?} to {toty:?}, message: {err}",
            )),
            Error::SerializeError(SerializeError {
                out_path: path,
                err_msg: msg,
                access: acc_ty,
            }) => {
                let acc_msg = match acc_ty {
                    AccessType::Write => "serialize",
                    AccessType::Read => "deserialize",
                };

                frmt.write_str(&format!(
                    "Could not {acc_msg} for file {path}, message: {msg}"
                ))
            }
            Error::FSError(FSError {
                file_name: file,
                err_msg: msg,
                access: acc_ty,
            }) => {
                let acc_msg = match acc_ty {
                    AccessType::Write => "write to",
                    AccessType::Read => "read from",
                };
                frmt.write_str(&format!("Could not {acc_msg} file {file}, message: {msg}"))
            }
            Error::PlantError(err) => fmt::Debug::fmt(err, frmt),
        }
    }
}
