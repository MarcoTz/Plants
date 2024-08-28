use super::bot_actions::{
    BotAction, FertilizePlants, MoveToGraveyard, NewActivity, NewGrowth, NewPlant, NewSpecies,
    Rain, UpdatePlant, UpdateSpecies, WaterLocation, WaterPlants,
};

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
}

pub enum CommandRes {
    NewAction(BotAction),
    Message(String),
}
pub fn handle_command(cmd: Command) -> CommandRes {
    match cmd {
        Command::Help => todo!(""),
        Command::Today => todo!(""),
        Command::Abort => todo!(""),
        Command::Push => todo!(""),
        Command::CheckLogs => todo!(""),
        Command::Water => CommandRes::NewAction(WaterPlants::default().into()),
        Command::WaterLocation => CommandRes::NewAction(WaterLocation::default().into()),
        Command::Fertilize => CommandRes::NewAction(FertilizePlants::default().into()),
        Command::Rain => CommandRes::NewAction(Rain::default().into()),
        Command::NewGrowth => CommandRes::NewAction(NewGrowth::default().into()),
        Command::NewPlant => CommandRes::NewAction(NewPlant::default().into()),
        Command::NewSpecies => CommandRes::NewAction(NewSpecies::default().into()),
        Command::NewActivity => CommandRes::NewAction(NewActivity::default().into()),
        Command::UpdateSpecies => CommandRes::NewAction(UpdateSpecies::default().into()),
        Command::UpdatePlant => CommandRes::NewAction(UpdatePlant::default().into()),
        Command::MoveToGraveyard => CommandRes::NewAction(MoveToGraveyard::default().into()),
    }
}
