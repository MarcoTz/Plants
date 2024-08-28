mod fertilize_plants;
mod input_handlers;
mod move_to_graveyard;
mod new_activity;
mod new_growth;
mod new_plant;
mod new_species;
mod rain;
mod update_plant;
mod update_species;
mod water_location;
mod water_plants;

use crate::errors::Error;
use database::database_manager::DatabaseManager;

pub use fertilize_plants::FertilizePlants;
pub use move_to_graveyard::MoveToGraveyard;
pub use new_activity::NewActivity;
pub use new_growth::NewGrowth;
pub use new_plant::NewPlant;
pub use new_species::NewSpecies;
pub use rain::Rain;
pub use update_plant::UpdatePlant;
pub use update_species::UpdateSpecies;
pub use water_location::WaterLocation;
pub use water_plants::WaterPlants;

pub enum BotAction {
    Idle,
    WaterPlants(WaterPlants),
    WaterLocation(WaterLocation),
    Rain(Rain),
    FertilizePlants(FertilizePlants),
    NewGrowth(NewGrowth),
    NewActivity(NewActivity),
    NewPlant(NewPlant),
    NewSpecies(NewSpecies),
    UpdateSpecies(UpdateSpecies),
    UpdatePlant(UpdatePlant),
    MoveToGraveyard(MoveToGraveyard),
}

impl PartialEq for BotAction {
    fn eq(&self, other: &BotAction) -> bool {
        match (self, other) {
            (BotAction::Idle, BotAction::Idle) => true,
            (BotAction::WaterPlants(_), BotAction::WaterPlants(_)) => true,
            (BotAction::WaterLocation(_), BotAction::WaterLocation(_)) => true,
            (BotAction::Rain(_), BotAction::Rain(_)) => true,
            (BotAction::FertilizePlants(_), BotAction::FertilizePlants(_)) => true,
            (BotAction::NewGrowth(_), BotAction::NewGrowth(_)) => true,
            (BotAction::NewActivity(_), BotAction::NewActivity(_)) => true,
            (BotAction::NewPlant(_), BotAction::NewPlant(_)) => true,
            (BotAction::NewSpecies(_), BotAction::NewSpecies(_)) => true,
            (BotAction::UpdateSpecies(_), BotAction::UpdateSpecies(_)) => true,
            (BotAction::UpdatePlant(_), BotAction::UpdatePlant(_)) => true,
            (BotAction::MoveToGraveyard(_), BotAction::MoveToGraveyard(_)) => true,
            _ => false,
        }
    }
}

impl Eq for BotAction {}

pub trait Action {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error>;
    fn is_done(&self) -> bool;
    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error>;
    fn get_next_prompt(&self) -> Result<String, Error>;
}

impl ToString for BotAction {
    fn to_string(&self) -> String {
        match self {
            BotAction::Idle => "Idle".to_owned(),
            BotAction::WaterPlants(_) => "Water Plants".to_owned(),
            BotAction::WaterLocation(_) => "Water Location".to_owned(),
            BotAction::Rain(_) => "Rain".to_owned(),
            BotAction::FertilizePlants(_) => "Fertilize Plants".to_owned(),
            BotAction::NewGrowth(_) => "New Growth".to_owned(),
            BotAction::NewActivity(_) => "New Activity".to_owned(),
            BotAction::NewPlant(_) => "New Plant".to_owned(),
            BotAction::NewSpecies(_) => "New Species".to_owned(),
            BotAction::UpdateSpecies(_) => "Update Species".to_owned(),
            BotAction::UpdatePlant(_) => "Update Plant".to_owned(),
            BotAction::MoveToGraveyard(_) => "Move To Graveyard".to_owned(),
        }
    }
}

impl Action for BotAction {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self {
            BotAction::Idle => Err(Error::NoActionRunning),
            BotAction::WaterPlants(water) => water.handle_input(input, db_man),
            BotAction::WaterLocation(water) => water.handle_input(input, db_man),
            BotAction::Rain(rain) => rain.handle_input(input, db_man),
            BotAction::FertilizePlants(fert) => fert.handle_input(input, db_man),
            BotAction::NewGrowth(growth) => growth.handle_input(input, db_man),
            BotAction::NewActivity(act) => act.handle_input(input, db_man),
            BotAction::NewPlant(newp) => newp.handle_input(input, db_man),
            BotAction::NewSpecies(newsp) => newsp.handle_input(input, db_man),
            BotAction::UpdateSpecies(updsp) => updsp.handle_input(input, db_man),
            BotAction::UpdatePlant(updpl) => updpl.handle_input(input, db_man),
            BotAction::MoveToGraveyard(gr) => gr.handle_input(input, db_man),
        }
    }
    fn is_done(&self) -> bool {
        match self {
            BotAction::Idle => true,
            BotAction::WaterPlants(water) => water.is_done(),
            BotAction::WaterLocation(water) => water.is_done(),
            BotAction::Rain(rain) => rain.is_done(),
            BotAction::FertilizePlants(fert) => fert.is_done(),
            BotAction::NewGrowth(growth) => growth.is_done(),
            BotAction::NewActivity(act) => act.is_done(),
            BotAction::NewPlant(newp) => newp.is_done(),
            BotAction::NewSpecies(newsp) => newsp.is_done(),
            BotAction::UpdateSpecies(updsp) => updsp.is_done(),
            BotAction::UpdatePlant(updpl) => updpl.is_done(),
            BotAction::MoveToGraveyard(gr) => gr.is_done(),
        }
    }
    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        match self {
            BotAction::Idle => Err(Error::NoActionRunning),
            BotAction::WaterPlants(water) => water.write_result(db_man),
            BotAction::WaterLocation(water) => water.write_result(db_man),
            BotAction::Rain(rain) => rain.write_result(db_man),
            BotAction::FertilizePlants(fert) => fert.write_result(db_man),
            BotAction::NewGrowth(growth) => growth.write_result(db_man),
            BotAction::NewActivity(act) => act.write_result(db_man),
            BotAction::NewPlant(newp) => newp.write_result(db_man),
            BotAction::NewSpecies(newsp) => newsp.write_result(db_man),
            BotAction::UpdateSpecies(updsp) => updsp.write_result(db_man),
            BotAction::UpdatePlant(updpl) => updpl.write_result(db_man),
            BotAction::MoveToGraveyard(gr) => gr.write_result(db_man),
        }
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        match self {
            BotAction::Idle => Err(Error::NoActionRunning),
            BotAction::WaterPlants(water) => water.get_next_prompt(),
            BotAction::WaterLocation(water) => water.get_next_prompt(),
            BotAction::Rain(rain) => rain.get_next_prompt(),
            BotAction::FertilizePlants(fert) => fert.get_next_prompt(),
            BotAction::NewGrowth(growth) => growth.get_next_prompt(),
            BotAction::NewActivity(act) => act.get_next_prompt(),
            BotAction::NewPlant(newp) => newp.get_next_prompt(),
            BotAction::NewSpecies(newsp) => newsp.get_next_prompt(),
            BotAction::UpdateSpecies(updsp) => updsp.get_next_prompt(),
            BotAction::UpdatePlant(updpl) => updpl.get_next_prompt(),
            BotAction::MoveToGraveyard(gr) => gr.get_next_prompt(),
        }
    }
}
