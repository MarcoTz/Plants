use std::fmt;

pub enum Error {
    SunlightError(String),
    GrowthError(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SunlightError(msg) => frmt.write_str(msg),
            Error::GrowthError(plant_name) => {
                frmt.write_str(&format!("Could not find growth for plant {}", plant_name))
            }
        }
    }
}
