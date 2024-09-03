use super::Error;
use serde_json::Value;
use std::fmt;

#[derive(Clone, Debug)]
pub struct WrongType {
    pub field_name: String,
    pub field_type: String,
}

#[derive(Clone, Debug)]
pub struct BadValue {
    pub field: String,
    pub val: Value,
}

#[derive(Clone, Debug)]
pub enum BadResponse {
    MissingField(String),
    BadValue(BadValue),
    WrongType(WrongType),
}

impl fmt::Display for WrongType {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        frmt.write_str(&format!(
            "Wrong Type for field {}, expected {}",
            self.field_name, self.field_type
        ))
    }
}

impl fmt::Display for BadValue {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        frmt.write_str(&format!(
            "Unexpected value {} for field {}",
            self.val, self.field
        ))
    }
}

impl fmt::Display for BadResponse {
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

#[cfg(test)]
mod bad_response_tests {
    use super::{BadResponse, BadValue, WrongType};
    use serde_json::Value;

    fn example_missing_field() -> BadResponse {
        BadResponse::MissingField("a field that should be present".to_owned())
    }

    fn example_bad_value() -> BadValue {
        BadValue {
            field: "ok".to_owned(),
            val: Value::Bool(false),
        }
    }

    fn example_wrong_type() -> WrongType {
        WrongType {
            field_name: "a bool field".to_owned(),
            field_type: "bool".to_owned(),
        }
    }

    #[test]
    fn display_missing_field() {
        let result = format!("{}", example_missing_field());
        let expected = "Missing field a field that should be present".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_bad_value() {
        let result = format!("{}", example_bad_value());
        let expected = "Unexpected value false for field ok".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_wrong_type() {
        let result = format!("{}", example_wrong_type());
        let expected = "Wrong Type for field a bool field, expected bool".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_into_bad_value() {
        let result = format!(
            "{}",
            <BadValue as Into<BadResponse>>::into(example_bad_value())
        );
        let expected = format!("{}", example_bad_value());
        assert_eq!(result, expected)
    }

    #[test]
    fn display_into_wrong_type() {
        let result = format!(
            "{}",
            <WrongType as Into<BadResponse>>::into(example_wrong_type())
        );
        let expected = format!("{}", example_wrong_type());
        assert_eq!(result, expected)
    }
}
