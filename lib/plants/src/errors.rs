use std::fmt;

#[derive(Debug)]
pub enum Error {
    SunlightError(String),
    GrowthError(String),
    EmptyVec(String),
    FieldError(String),
    WrongType(String),
    SpeciesNotFound(String),
    BadHealth(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SunlightError(msg) => frmt.write_str(msg),
            Error::GrowthError(plant_name) => {
                write!(frmt, "Could not find growth for plant {plant_name}")
            }
            Error::EmptyVec(msg) => write!(frmt, "No plants provided, message: {msg}"),
            Error::FieldError(field) => write!(frmt, "Cannot update {field}"),
            Error::WrongType(field) => write!(frmt, "Wrong type for {field}"),
            Error::SpeciesNotFound(name) => write!(frmt, "Could not find species {name}"),
            Error::BadHealth(num) => write!(frmt, "{num} is not a valid value for health"),
        }
    }
}

#[cfg(test)]
mod error_tests {
    use super::Error;

    #[test]
    fn display_sunlight() {
        let result = format!("{}", Error::SunlightError("bad value".to_owned()));
        let expected = "bad value";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_growth() {
        let result = format!("{}", Error::GrowthError("a plant".to_owned()));
        let expected = "Could not find growth for plant a plant";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_empty() {
        let result = format!("{}", Error::EmptyVec("a message".to_owned()));
        let expected = "No plants provided, message: a message";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_field() {
        let result = format!("{}", Error::FieldError("a field".to_owned()));
        let expected = "Cannot update a field";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_type() {
        let result = format!("{}", Error::WrongType("a field".to_owned()));
        let expected = "Wrong type for a field";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_species() {
        let result = format!("{}", Error::SpeciesNotFound("not a species".to_owned()));
        let expected = "Could not find species not a species";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_health() {
        let result = format!("{}", Error::BadHealth(6));
        let expected = "6 is not a valid value for health";
        assert_eq!(result, expected)
    }
}
