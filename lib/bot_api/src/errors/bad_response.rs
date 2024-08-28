use super::Error;
use serde_json::Value;
use std::fmt;

#[derive(Clone)]
pub struct WrongType {
    pub field_name: String,
    pub field_type: String,
}

#[derive(Clone)]
pub struct BadValue {
    pub field: String,
    pub val: Value,
}

#[derive(Clone)]
pub enum BadResponse {
    MissingField(String),
    BadValue(BadValue),
    WrongType(WrongType),
}

impl fmt::Debug for WrongType {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        frmt.write_str(&format!(
            "Wrong Type for feld {}, expected {}",
            self.field_name, self.field_type
        ))
    }
}

impl fmt::Debug for BadValue {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        frmt.write_str(&format!(
            "Unexpeced value {} for field {}",
            self.val, self.field
        ))
    }
}

impl fmt::Debug for BadResponse {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BadResponse::MissingField(field_name) => {
                frmt.write_str(&format!("Missing field {field_name}"))
            }
            BadResponse::BadValue(bad_val) => bad_val.fmt(frmt),
            BadResponse::WrongType(wrong_ty) => wrong_ty.fmt(frmt),
        }
    }
}

impl From<BadResponse> for Error {
    fn from(bad_resp: BadResponse) -> Error {
        Error::BadResponse(bad_resp)
    }
}

impl From<WrongType> for BadResponse {
    fn from(wrong_ty: WrongType) -> BadResponse {
        BadResponse::WrongType(wrong_ty)
    }
}

impl From<BadValue> for BadResponse {
    fn from(bad_val: BadValue) -> BadResponse {
        BadResponse::BadValue(bad_val)
    }
}
