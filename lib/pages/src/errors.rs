use std::fmt;

#[derive(Debug)]
pub enum Error {
    PlantError(plants::errors::Error),
    LocationError(Vec<String>),
}

impl From<plants::errors::Error> for Error {
    fn from(plant_err: plants::errors::Error) -> Error {
        Error::PlantError(plant_err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PlantError(plant_err) => plant_err.fmt(frmt),
            Error::LocationError(locations) => frmt.write_str(&format!(
                "Cannot create location group for multiple locations {locations:?}"
            )),
        }
    }
}
