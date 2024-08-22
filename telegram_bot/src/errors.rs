use database::errors::Error as DBError;
use std::fmt;

pub enum Error {
    DBError(DBError),
    ActionAlreadyDone(String),
    NoPlantsLocation(String),
    PlantDoesNotExist(String),
    ParseError(String),
    BadHealth(i32),
    MissingInput(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DBError(db_err) => db_err.fmt(frmt),
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
        }
    }
}

impl From<DBError> for Error {
    fn from(db_err: DBError) -> Error {
        Error::DBError(db_err)
    }
}
