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

#[cfg(test)]
mod error_tests {
    use super::Error;
    use plants::errors::Error as PlantError;

    fn example_plant_err() -> PlantError {
        PlantError::SunlightError("light".to_owned())
    }

    #[test]
    fn display_plant_err() {
        let result = format!("{}", Error::PlantError(example_plant_err()));
        let expected = "light";
        assert_eq!(result, expected);
    }

    #[test]
    fn display_into() {
        let result = format!("{}", <PlantError as Into<Error>>::into(example_plant_err()));
        let expected = format!("{}", example_plant_err());
        assert_eq!(result, expected)
    }

    #[test]
    fn display_location() {
        let result = format!(
            "{}",
            Error::LocationError(vec!["Location1".to_owned(), "Location2".to_owned()])
        );
        let expected =
            "Cannot create location group for multiple locations [\"Location1\", \"Location2\"]";
        assert_eq!(result, expected)
    }
}
