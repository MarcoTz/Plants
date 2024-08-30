use super::{
    action_handler::ImmediateAction,
    bot_actions::{
        BotAction, FertilizePlants, MoveToGraveyard, NewActivity, NewGrowth, NewPlant, NewSpecies,
        Rain, UpdatePlant, UpdateSpecies, WaterLocation, WaterPlants,
    },
    errors::Error,
};
use bot_api::commands::Command as BotCommand;
use chrono::Local;
use std::{fmt, str};

pub enum Command {
    Help,
    Water,
    WaterLocation,
    Fertilize,
    Rain,
    NewGrowth,
    NewPlant,
    NewSpecies,
    NewActivity,
    UpdateSpecies,
    UpdatePlant,
    Today,
    MoveToGraveyard,
    Abort,
    Push,
    CheckLogs,
    Exit,
}

pub enum CommandRes {
    NewAction(Box<BotAction>),
    NewInput(String),
    ImmediateAction(ImmediateAction),
    Message(String),
}

impl Command {
    fn get_all() -> Vec<Command> {
        vec![
            Command::Help,
            Command::Water,
            Command::WaterLocation,
            Command::Fertilize,
            Command::Rain,
            Command::NewGrowth,
            Command::NewPlant,
            Command::NewSpecies,
            Command::NewActivity,
            Command::UpdateSpecies,
            Command::UpdatePlant,
            Command::Today,
            Command::MoveToGraveyard,
            Command::Abort,
            Command::Push,
            Command::CheckLogs,
            Command::Exit,
        ]
    }

    pub fn get_res(&self) -> CommandRes {
        match self {
            Command::Help => {
                let all_commands = Command::get_all();
                let help_lines: Vec<String> = all_commands
                    .iter()
                    .map(|cmd| cmd.to_string() + " -- " + &cmd.get_description())
                    .collect();
                let help_str = format!("Possible commands: \n\n {}", help_lines.join("\n"));
                CommandRes::Message(help_str)
            }
            Command::Today => {
                CommandRes::NewInput(Local::now().date_naive().format("%d.%m.%Y").to_string())
            }
            Command::Abort => CommandRes::NewAction(Box::new(BotAction::Idle)),
            Command::Push => CommandRes::ImmediateAction(ImmediateAction::Push),
            Command::CheckLogs => CommandRes::ImmediateAction(ImmediateAction::CheckLogs),
            Command::Water => CommandRes::NewAction(Box::new(WaterPlants::default().into())),
            Command::WaterLocation => {
                CommandRes::NewAction(Box::new(WaterLocation::default().into()))
            }
            Command::Fertilize => {
                CommandRes::NewAction(Box::new(FertilizePlants::default().into()))
            }
            Command::Rain => CommandRes::NewAction(Box::new(Rain.into())),
            Command::NewGrowth => CommandRes::NewAction(Box::new(NewGrowth::default().into())),
            Command::NewPlant => CommandRes::NewAction(Box::new(NewPlant::default().into())),
            Command::NewSpecies => CommandRes::NewAction(Box::new(NewSpecies::default().into())),
            Command::NewActivity => CommandRes::NewAction(Box::new(NewActivity::default().into())),
            Command::UpdateSpecies => {
                CommandRes::NewAction(Box::new(UpdateSpecies::default().into()))
            }
            Command::UpdatePlant => CommandRes::NewAction(Box::new(UpdatePlant::default().into())),
            Command::MoveToGraveyard => {
                CommandRes::NewAction(Box::new(MoveToGraveyard::default().into()))
            }
            Command::Exit => CommandRes::ImmediateAction(ImmediateAction::Exit),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Help => frmt.write_str("help"),
            Command::Today => frmt.write_str("today"),
            Command::Abort => frmt.write_str("abort"),
            Command::Push => frmt.write_str("push"),
            Command::CheckLogs => frmt.write_str("check_logs"),
            Command::Water => frmt.write_str("water"),
            Command::WaterLocation => frmt.write_str("water_location"),
            Command::Fertilize => frmt.write_str("fertilize"),
            Command::Rain => frmt.write_str("rain"),
            Command::NewGrowth => frmt.write_str("new_growth"),
            Command::NewPlant => frmt.write_str("new_plant"),
            Command::NewSpecies => frmt.write_str("new_species"),
            Command::NewActivity => frmt.write_str("new_activity"),
            Command::UpdateSpecies => frmt.write_str("update_species"),
            Command::UpdatePlant => frmt.write_str("update_plant"),
            Command::MoveToGraveyard => frmt.write_str("move_to_graveyard"),
            Command::Exit => frmt.write_str("exit"),
        }
    }
}

impl str::FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Command, Self::Err> {
        match s {
            "help" => Ok(Command::Help),
            "today" => Ok(Command::Today),
            "abort" => Ok(Command::Abort),
            "push" => Ok(Command::Push),
            "check_logs" => Ok(Command::CheckLogs),
            "water" => Ok(Command::Water),
            "water_location" => Ok(Command::WaterLocation),
            "fertilize" => Ok(Command::Fertilize),
            "rain" => Ok(Command::Rain),
            "new_growth" => Ok(Command::NewGrowth),
            "new_plant" => Ok(Command::NewPlant),
            "new_species" => Ok(Command::NewSpecies),
            "new_activity" => Ok(Command::NewActivity),
            "update_species" => Ok(Command::UpdateSpecies),
            "update_plant" => Ok(Command::UpdatePlant),
            "move_to_graveyard" => Ok(Command::MoveToGraveyard),
            "exit" => Ok(Command::Exit),
            _ => Err(Error::ParseError(format!("Command {s}"))),
        }
    }
}

impl BotCommand for Command {
    type Error = Error;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        s.parse::<Command>()
    }
    fn get_description(&self) -> String {
        match self {
            Command::Help => "Display Help Text".to_owned(),
            Command::Water => "Water plants (today)".to_owned(),
            Command::WaterLocation => "Water all plants in location (today)".to_owned(),
            Command::Fertilize => "Fertilize plants (today)".to_owned(),
            Command::Rain => "It was rained (all outside plants will be watered)".to_owned(),
            Command::NewGrowth => "Enter new growth".to_owned(),
            Command::NewPlant => "Enter new plant".to_owned(),
            Command::NewSpecies => "Enter new species".to_owned(),
            Command::NewActivity => "Enter new activity".to_owned(),
            Command::UpdateSpecies => "Update species".to_owned(),
            Command::UpdatePlant => "Update plant".to_owned(),
            Command::Today => "Enter the current date as input".to_owned(),
            Command::MoveToGraveyard => "Move Plant to graveyard".to_owned(),
            Command::Abort => "Abort the current action".to_owned(),
            Command::Push => "Push local changes to github".to_owned(),
            Command::CheckLogs => "Check warnings generated from build".to_owned(),
            Command::Exit => "Exit the bot".to_owned(),
        }
    }
}
