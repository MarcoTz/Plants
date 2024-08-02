use std::fmt;

pub enum PlantError {
    SunlightError(String),
}

impl fmt::Debug for PlantError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlantError::SunlightError(msg) => frmt.write_str(msg),
        }
    }
}
