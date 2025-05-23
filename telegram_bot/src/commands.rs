use super::{
    action_handler::ImmediateAction,
    bot_actions::{
        BotAction, FertilizePlants, GetLocationPlants, GetPlantActivities, GetPlantDetails,
        GetPlantFertilizing, GetPlantGrowth, GetPlantWatering, GetSpeciesDetails, MoveToGraveyard,
        NewActivity, NewGrowth, NewPlant, NewSpecies, Rain, UpdatePlant, UpdateSpecies,
        WaterLocation, WaterPlants,
    },
    errors::Error,
};
use bot_api::commands::Command as BotCommand;
use chrono::Local;
use std::{fmt, str};

#[derive(Debug, PartialEq, Eq)]
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
    TodayWater,
    TodayFertilize,
    TodayGrowth,

    AllPlants,
    AllSpecies,
    LocationPlants,
    PlantDetails,
    SpeciesDetails,
    PlantActivities,
    PlantWatering,
    PlantFertilizing,
    PlantGrowth,
}

#[derive(Debug, PartialEq, Eq)]
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
            Command::TodayWater,
            Command::TodayFertilize,
            Command::TodayGrowth,
            Command::AllPlants,
            Command::AllSpecies,
            Command::LocationPlants,
            Command::PlantDetails,
            Command::SpeciesDetails,
            Command::PlantActivities,
            Command::PlantWatering,
            Command::PlantFertilizing,
            Command::PlantGrowth,
        ]
    }

    pub fn get_res(&self) -> CommandRes {
        match self {
            Command::Help => {
                let all_commands = Command::get_all();
                let help_lines: Vec<String> = all_commands
                    .iter()
                    .map(|cmd| format!("/{} -- {}", cmd, cmd.get_description()))
                    .collect();
                let help_str = format!("Possible commands:\n\n{}", help_lines.join("\n"));
                CommandRes::Message(help_str)
            }
            Command::Today => {
                CommandRes::NewInput(Local::now().date_naive().format("%d.%m.%Y").to_string())
            }
            Command::Abort => CommandRes::ImmediateAction(ImmediateAction::Abort),
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
            Command::TodayWater => CommandRes::ImmediateAction(ImmediateAction::GetWaterToday),
            Command::TodayFertilize => {
                CommandRes::ImmediateAction(ImmediateAction::GetFertilizeToday)
            }
            Command::TodayGrowth => CommandRes::ImmediateAction(ImmediateAction::GetGrowthToday),
            Command::AllPlants => CommandRes::ImmediateAction(ImmediateAction::GetAllPlants),
            Command::LocationPlants => {
                CommandRes::NewAction(Box::new(GetLocationPlants::default().into()))
            }
            Command::AllSpecies => CommandRes::ImmediateAction(ImmediateAction::GetAllSpecies),
            Command::PlantDetails => {
                CommandRes::NewAction(Box::new(GetPlantDetails::default().into()))
            }
            Command::SpeciesDetails => {
                CommandRes::NewAction(Box::new(GetSpeciesDetails::default().into()))
            }
            Command::PlantActivities => {
                CommandRes::NewAction(Box::new(GetPlantActivities::default().into()))
            }
            Command::PlantWatering => {
                CommandRes::NewAction(Box::new(GetPlantWatering::default().into()))
            }
            Command::PlantFertilizing => {
                CommandRes::NewAction(Box::new(GetPlantFertilizing::default().into()))
            }
            Command::PlantGrowth => {
                CommandRes::NewAction(Box::new(GetPlantGrowth::default().into()))
            }
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Help => f.write_str("help"),
            Command::Today => f.write_str("today"),
            Command::Abort => f.write_str("abort"),
            Command::Push => f.write_str("push"),
            Command::CheckLogs => f.write_str("check_logs"),
            Command::Water => f.write_str("water"),
            Command::WaterLocation => f.write_str("water_location"),
            Command::Fertilize => f.write_str("fertilize"),
            Command::Rain => f.write_str("rain"),
            Command::NewGrowth => f.write_str("new_growth"),
            Command::NewPlant => f.write_str("new_plant"),
            Command::NewSpecies => f.write_str("new_species"),
            Command::NewActivity => f.write_str("new_activity"),
            Command::UpdateSpecies => f.write_str("update_species"),
            Command::UpdatePlant => f.write_str("update_plant"),
            Command::MoveToGraveyard => f.write_str("move_to_graveyard"),
            Command::TodayWater => f.write_str("today_water"),
            Command::TodayFertilize => f.write_str("today_fertilize"),
            Command::TodayGrowth => f.write_str("today_growth"),
            Command::AllPlants => f.write_str("all_plants"),
            Command::LocationPlants => f.write_str("loction_plants"),
            Command::AllSpecies => f.write_str("all_species"),
            Command::PlantDetails => f.write_str("plant_details"),
            Command::SpeciesDetails => f.write_str("species_details"),
            Command::PlantActivities => f.write_str("plant_activities"),
            Command::PlantWatering => f.write_str("plant_watering"),
            Command::PlantFertilizing => f.write_str("plant_fertilizing"),
            Command::PlantGrowth => f.write_str("plant_growth"),
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
            "today_water" => Ok(Command::TodayWater),
            "today_fertilize" => Ok(Command::TodayFertilize),
            "today_growth" => Ok(Command::TodayGrowth),
            "all_plants" => Ok(Command::AllPlants),
            "loction_plants" => Ok(Command::LocationPlants),
            "all_species" => Ok(Command::AllSpecies),
            "plant_details" => Ok(Command::PlantDetails),
            "species_details" => Ok(Command::SpeciesDetails),
            "plant_activities" => Ok(Command::PlantActivities),
            "plant_watering" => Ok(Command::PlantWatering),
            "plant_fertilizing" => Ok(Command::PlantFertilizing),
            "plant_growth" => Ok(Command::PlantGrowth),
            _ => Err(Error::ParseError(format!("Command {s}"))),
        }
    }
}

impl BotCommand for Command {
    fn parse(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        s.parse::<Command>().map_err(|err| err.into())
    }
    fn get_description(&self) -> String {
        match self {
            Command::Help => "Display Help Text".to_owned(),
            Command::Water => "Water plants (today)".to_owned(),
            Command::WaterLocation => "Water all plants in location (today)".to_owned(),
            Command::Fertilize => "Fertilize plants (today)".to_owned(),
            Command::Rain => "It rained (all outside plants will be watered)".to_owned(),
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
            Command::TodayWater => "Check which plants should get watered today".to_owned(),
            Command::TodayFertilize => "Check which plants should be fertilized today".to_owned(),
            Command::TodayGrowth => "Check which plants need growth updates by today".to_owned(),
            Command::AllPlants => "Show all plants".to_owned(),
            Command::LocationPlants => "Show plants at location".to_owned(),
            Command::AllSpecies => "Show all species".to_owned(),
            Command::PlantDetails => "Show details for plant".to_owned(),
            Command::SpeciesDetails => "Show details for species".to_owned(),
            Command::PlantActivities => "Show Activities for plant".to_owned(),
            Command::PlantWatering => "Show last watering days for plant".to_owned(),
            Command::PlantFertilizing => "Show last fertilizing days for plant".to_owned(),
            Command::PlantGrowth => "Show growht updates for plant".to_owned(),
        }
    }
}

#[cfg(test)]
mod command_tests {
    use super::{
        BotAction, BotCommand, Command, CommandRes, FertilizePlants, ImmediateAction,
        MoveToGraveyard, NewActivity, NewGrowth, NewPlant, NewSpecies, Rain, UpdatePlant,
        UpdateSpecies, WaterLocation, WaterPlants,
    };
    use chrono::Local;
    use std::str::FromStr;

    #[test]
    fn result_water() {
        let result = Command::Water.get_res();
        let expected =
            CommandRes::NewAction(Box::new(BotAction::WaterPlants(WaterPlants::default())));
        assert_eq!(result, expected)
    }

    #[test]
    fn result_waterloc() {
        let result = Command::WaterLocation.get_res();
        let expected =
            CommandRes::NewAction(Box::new(BotAction::WaterLocation(WaterLocation::default())));
        assert_eq!(result, expected)
    }

    #[test]
    fn result_fertilize() {
        let result = Command::Fertilize.get_res();
        let expected = CommandRes::NewAction(Box::new(BotAction::FertilizePlants(
            FertilizePlants::default(),
        )));
        assert_eq!(result, expected)
    }

    #[test]
    fn result_rain() {
        let result = Command::Rain.get_res();
        let expected = CommandRes::NewAction(Box::new(BotAction::Rain(Rain::default())));
        assert_eq!(result, expected)
    }

    #[test]
    fn result_newgrowth() {
        let result = Command::NewGrowth.get_res();
        let expected = CommandRes::NewAction(Box::new(BotAction::NewGrowth(NewGrowth::default())));
        assert_eq!(result, expected)
    }

    #[test]
    fn result_newplant() {
        let result = Command::NewPlant.get_res();
        let expected = CommandRes::NewAction(Box::new(BotAction::NewPlant(NewPlant::default())));
        assert_eq!(result, expected)
    }

    #[test]
    fn result_newspecies() {
        let result = Command::NewSpecies.get_res();
        let expected =
            CommandRes::NewAction(Box::new(BotAction::NewSpecies(NewSpecies::default())));
        assert_eq!(result, expected)
    }

    #[test]
    fn result_newactivity() {
        let result = Command::NewActivity.get_res();
        let expected =
            CommandRes::NewAction(Box::new(BotAction::NewActivity(NewActivity::default())));
        assert_eq!(result, expected)
    }

    #[test]
    fn resutl_updatespecies() {
        let result = Command::UpdateSpecies.get_res();
        let expected =
            CommandRes::NewAction(Box::new(BotAction::UpdateSpecies(UpdateSpecies::default())));
        assert_eq!(result, expected)
    }

    #[test]
    fn result_updateplant() {
        let result = Command::UpdatePlant.get_res();
        let expected =
            CommandRes::NewAction(Box::new(BotAction::UpdatePlant(UpdatePlant::default())));
        assert_eq!(result, expected)
    }

    #[test]
    fn result_today() {
        let result = Command::Today.get_res();
        let expected =
            CommandRes::NewInput(Local::now().date_naive().format("%d.%m.%Y").to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn result_movegraveyard() {
        let result = Command::MoveToGraveyard.get_res();
        let expected = CommandRes::NewAction(Box::new(BotAction::MoveToGraveyard(
            MoveToGraveyard::default(),
        )));
        assert_eq!(result, expected)
    }

    #[test]
    fn result_abort() {
        let result = Command::Abort.get_res();
        let expected = CommandRes::ImmediateAction(ImmediateAction::Abort);
        assert_eq!(result, expected)
    }

    #[test]
    fn result_push() {
        let result = Command::Push.get_res();
        let expected = CommandRes::ImmediateAction(ImmediateAction::Push);
        assert_eq!(result, expected)
    }

    #[test]
    fn result_checklogs() {
        let result = Command::CheckLogs.get_res();
        let expected = CommandRes::ImmediateAction(ImmediateAction::CheckLogs);
        assert_eq!(result, expected)
    }

    #[test]
    fn display_help() {
        let result = format!("{}", Command::Help);
        let expected = "help";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_today() {
        let result = format!("{}", Command::Today);
        let expected = "today";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_abort() {
        let result = format!("{}", Command::Abort);
        let expected = "abort";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_push() {
        let result = format!("{}", Command::Push);
        let expected = "push";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_checklogs() {
        let result = format!("{}", Command::CheckLogs);
        let expected = "check_logs";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_water() {
        let result = format!("{}", Command::Water);
        let expected = "water";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_waterloc() {
        let result = format!("{}", Command::WaterLocation);
        let expected = "water_location";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_fertilize() {
        let result = format!("{}", Command::Fertilize);
        let expected = "fertilize";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_rain() {
        let result = format!("{}", Command::Rain);
        let expected = "rain";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_newgrowth() {
        let result = format!("{}", Command::NewGrowth);
        let expected = "new_growth";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_newplant() {
        let result = format!("{}", Command::NewPlant);
        let expected = "new_plant";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_newactivity() {
        let result = format!("{}", Command::NewActivity);
        let expected = "new_activity";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_newspecies() {
        let result = format!("{}", Command::NewSpecies);
        let expected = "new_species";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_updateplant() {
        let result = format!("{}", Command::UpdatePlant);
        let expected = "update_plant";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_updatespecies() {
        let result = format!("{}", Command::UpdateSpecies);
        let expected = "update_species";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_movegraveyard() {
        let result = format!("{}", Command::MoveToGraveyard);
        let expected = "move_to_graveyard";
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_help() {
        let result = Command::from_str("help").unwrap();
        let expected = Command::Help;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_today() {
        let result = Command::from_str("today").unwrap();
        let expected = Command::Today;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_abort() {
        let result = Command::from_str("abort").unwrap();
        let expected = Command::Abort;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_push() {
        let result = Command::from_str("push").unwrap();
        let expected = Command::Push;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_check_logs() {
        let result = Command::from_str("check_logs").unwrap();
        let expected = Command::CheckLogs;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_water() {
        let result = Command::from_str("water").unwrap();
        let expected = Command::Water;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_waterloc() {
        let result = Command::from_str("water_location").unwrap();
        let expected = Command::WaterLocation;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_fert() {
        let result = Command::from_str("fertilize").unwrap();
        let expected = Command::Fertilize;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_rain() {
        let result = Command::from_str("rain").unwrap();
        let expected = Command::Rain;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_newgrowth() {
        let result = Command::from_str("new_growth").unwrap();
        let expected = Command::NewGrowth;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_newplant() {
        let result = Command::from_str("new_plant").unwrap();
        let expected = Command::NewPlant;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_newspecies() {
        let result = Command::from_str("new_species").unwrap();
        let expected = Command::NewSpecies;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_newactivity() {
        let result = Command::from_str("new_activity").unwrap();
        let expected = Command::NewActivity;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_updateplant() {
        let result = Command::from_str("update_plant").unwrap();
        let expected = Command::UpdatePlant;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_updatespecies() {
        let result = Command::from_str("update_species").unwrap();
        let expected = Command::UpdateSpecies;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_movegraveyard() {
        let result = Command::from_str("move_to_graveyard").unwrap();
        let expected = Command::MoveToGraveyard;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_str_err() {
        let result = Command::from_str("other");
        assert!(result.is_err())
    }

    #[test]
    fn botcmd_from_str() {
        let result = <Command as BotCommand>::parse("help").unwrap();
        let expected = Command::Help;
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_help() {
        let result = Command::Help.get_description();
        let expected = "Display Help Text";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_water() {
        let result = Command::Water.get_description();
        let expected = "Water plants (today)";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_waterloc() {
        let result = Command::WaterLocation.get_description();
        let expected = "Water all plants in location (today)";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_fertilize() {
        let result = Command::Fertilize.get_description();
        let expected = "Fertilize plants (today)";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_rain() {
        let result = Command::Rain.get_description();
        let expected = "It rained (all outside plants will be watered)";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_newgrowth() {
        let result = Command::NewGrowth.get_description();
        let expected = "Enter new growth";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_newplant() {
        let result = Command::NewPlant.get_description();
        let expected = "Enter new plant";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_newspecies() {
        let result = Command::NewSpecies.get_description();
        let expected = "Enter new species";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_newactivity() {
        let result = Command::NewActivity.get_description();
        let expected = "Enter new activity";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_today() {
        let result = Command::Today.get_description();
        let expected = "Enter the current date as input";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_updateplant() {
        let result = Command::UpdatePlant.get_description();
        let expected = "Update plant";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_updatespecies() {
        let result = Command::UpdateSpecies.get_description();
        let expected = "Update species";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_movegraveyard() {
        let result = Command::MoveToGraveyard.get_description();
        let expected = "Move Plant to graveyard";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_abort() {
        let result = Command::Abort.get_description();
        let expected = "Abort the current action";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_push() {
        let result = Command::Push.get_description();
        let expected = "Push local changes to github";
        assert_eq!(result, expected)
    }

    #[test]
    fn desc_checklogs() {
        let result = Command::CheckLogs.get_description();
        let expected = "Check warnings generated from build";
        assert_eq!(result, expected)
    }
}
