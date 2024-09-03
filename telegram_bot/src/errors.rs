use bot_api::errors::Error as BotError;
use plants::errors::Error as PlantError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    PlantError(PlantError),
    BotError(BotError),
    NoActionRunning,
    ActionAlreadyRunning(String),
    ActionAlreadyDone(String),
    NoPlantsLocation(String),
    PlantDoesNotExist(String),
    ParseError(String),
    BadHealth(i32),
    MissingInput(String),
    PlantExists(String),
    SpeciesDoesNotExist(String),
    SpeciesExists(String),
    Other(Box<dyn std::error::Error>),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PlantError(plant_err) => frmt.write_str(&format!("{plant_err:?}")),
            Error::BotError(bot_err) => frmt.write_str(&format!("{bot_err:?}")),
            Error::NoActionRunning => {
                frmt.write_str("Currently there is no active action, please try again")
            }
            Error::ActionAlreadyRunning(action) => frmt.write_str(&format!(
                "Action {action} is already running, please either complete or abort"
            )),
            Error::ActionAlreadyDone(action) => frmt.write_str(&format!(
                "Action {action} is already done, cannot handle more input"
            )),
            Error::NoPlantsLocation(loc) => {
                frmt.write_str(&format!("Location {loc} does not have any plants"))
            }
            Error::PlantDoesNotExist(plant) => {
                frmt.write_str(&format!("Plant {plant} does not exist"))
            }
            Error::ParseError(msg) => frmt.write_str(&format!("Could not parse {msg}")),
            Error::BadHealth(health) => {
                frmt.write_str(&format!("{health} is not a valid value for plant health"))
            }
            Error::MissingInput(msg) => {
                frmt.write_str(&format!("Input {msg} is missing, please try again"))
            }
            Error::PlantExists(name) => frmt.write_str(&format!("Plant {name} already exists")),
            Error::SpeciesDoesNotExist(name) => {
                frmt.write_str(&format!("Species {name} does not exist"))
            }
            Error::SpeciesExists(name) => frmt.write_str(&format!("Species {name} already exists")),
            Error::Other(err) => err.fmt(frmt),
        }
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Error {
        Error::Other(err)
    }
}

impl From<PlantError> for Error {
    fn from(plant_err: PlantError) -> Error {
        Error::PlantError(plant_err)
    }
}

impl From<BotError> for Error {
    fn from(bot_err: BotError) -> Error {
        Error::BotError(bot_err)
    }
}
