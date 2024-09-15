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
    Unauthorized(String),
    Command(CommandError),
    Logger,
    Other(Box<dyn std::error::Error>),
}

#[derive(Debug)]
pub struct CommandError {
    pub cmd: String,
    pub msg: String,
}

impl From<CommandError> for Error {
    fn from(cmd_err: CommandError) -> Error {
        Error::Command(cmd_err)
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command {} existed with messge {}", self.cmd, self.msg)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PlantError(plant_err) => frmt.write_str(&format!("{plant_err}")),
            Error::BotError(bot_err) => frmt.write_str(&format!("{bot_err}")),
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
            Error::Unauthorized(name) => write!(frmt, "User {name} is not authorized"),
            Error::Logger => write!(frmt, "Could not initialize logger"),
            Error::Command(msg) => msg.fmt(frmt),
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

#[cfg(test)]
mod error_tests {
    use super::{CommandError, Error};
    use bot_api::errors::Error as BotErr;
    use bot_api::errors::ParseError;
    use plants::errors::Error as PlantErr;

    #[test]
    fn display_plant_err() {
        let result = format!(
            "{}",
            Error::PlantError(PlantErr::SpeciesNotFound("not a species".to_owned()))
        );
        let expected = "Could not find species not a species";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_bot_err() {
        let result = format!(
            "{}",
            Error::BotError(BotErr::Parse(ParseError {
                ty: "date".to_owned()
            }))
        );
        let expected = "Could not parse date";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_noaction() {
        let result = format!("{}", Error::NoActionRunning);
        let expected = "Currently there is no active action, please try again";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_alreadydone() {
        let result = format!("{}", Error::ActionAlreadyDone("Idle".to_owned()));
        let expected = "Action Idle is already done, cannot handle more input";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_noplantsloc() {
        let result = format!("{}", Error::NoPlantsLocation("location".to_owned()));
        let expected = "Location location does not have any plants";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_badhealth() {
        let result = format!("{}", Error::BadHealth(6));
        let expected = "6 is not a valid value for plant health";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_missinginp() {
        let result = format!("{}", Error::MissingInput("plant".to_owned()));
        let expected = "Input plant is missing, please try again";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_plantexists() {
        let result = format!("{}", Error::PlantExists("plant".to_owned()));
        let expected = "Plant plant already exists";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_speciesnotexist() {
        let result = format!("{}", Error::SpeciesDoesNotExist("species".to_owned()));
        let expected = "Species species does not exist";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_speciesexists() {
        let result = format!("{}", Error::SpeciesExists("species".to_owned()));
        let expected = "Species species already exists";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_unauthorized() {
        let result = format!("{}", Error::Unauthorized("username".to_owned()));
        let expected = "User username is not authorized";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_logger() {
        let result = format!("{}", Error::Logger);
        let expected = "Could not initialize logger";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_other() {
        let result = format!(
            "{}",
            Error::Other(Box::new(PlantErr::WrongType("obtained".to_owned())))
        );
        let expected = "Wrong type for obtained";
        assert_eq!(result, expected)
    }

    #[test]
    fn bot_into() {
        let result = format!(
            "{}",
            <BotErr as Into<Error>>::into(BotErr::Parse(ParseError {
                ty: "Int".to_owned()
            }))
        );
        let expected = format!(
            "{}",
            Error::BotError(BotErr::Parse(ParseError {
                ty: "Int".to_owned()
            }))
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn box_into() {
        let result = format!(
            "{}",
            <Box<dyn std::error::Error> as Into<Error>>::into(Box::new(PlantErr::FieldError(
                "field".to_owned()
            )))
        );
        let expected = format!(
            "{}",
            Error::Other(Box::new(PlantErr::FieldError("field".to_owned())))
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn plant_into() {
        let result = format!(
            "{}",
            <PlantErr as Into<Error>>::into(PlantErr::FieldError("field".to_owned()))
        );
        let expected = format!(
            "{}",
            Error::PlantError(PlantErr::FieldError("field".to_owned()))
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn display_cmd_err() {
        let result = format!(
            "{}",
            CommandError {
                cmd: "git add -A".to_owned(),
                msg: "fail".to_owned()
            }
        );
        let expected = "Command git add -A exited with error fail";
        assert_eq!(result, expected)
    }

    #[test]
    fn cmd_into() {
        let result = format!(
            "{}",
            <CommandError as Into<Error>>::into(CommandError {
                cmd: "git add -A".to_owned(),
                msg: "fail".to_owned()
            })
        );
        let expected = format!(
            "{}",
            CommandError {
                cmd: "git add -A".to_owned(),
                msg: "fail".to_owned()
            }
        );
        assert_eq!(result, expected)
    }
}
