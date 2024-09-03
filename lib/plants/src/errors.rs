use std::fmt;

#[derive(Debug)]
pub enum Error {
    SunlightError(String),
    GrowthError(String),
    EmptyVec(String),
    FieldError(String),
    WrongType(String),
}

impl fmt::Display for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SunlightError(msg) => frmt.write_str(msg),
            Error::GrowthError(plant_name) => {
                frmt.write_str(&format!("Could not find growth for plant {plant_name}"))
            }
            Error::EmptyVec(msg) => frmt.write_str(&format!("No plants provided, message; {msg}")),
            Error::FieldError(field) => frmt.write_str(&format!("Cannot update {field}")),
            Error::WrongType(field) => frmt.write_str(&format!("Wrong type for {field}")),
        }
    }
}
