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
use std::fmt;

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

#[derive(Clone)]
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
        matches!(
            (self, other),
            (BotAction::Idle, BotAction::Idle)
                | (BotAction::WaterPlants(_), BotAction::WaterPlants(_))
                | (BotAction::WaterLocation(_), BotAction::WaterLocation(_))
                | (BotAction::Rain(_), BotAction::Rain(_))
                | (BotAction::FertilizePlants(_), BotAction::FertilizePlants(_))
                | (BotAction::NewGrowth(_), BotAction::NewGrowth(_))
                | (BotAction::NewActivity(_), BotAction::NewActivity(_))
                | (BotAction::NewPlant(_), BotAction::NewPlant(_))
                | (BotAction::NewSpecies(_), BotAction::NewSpecies(_))
                | (BotAction::UpdateSpecies(_), BotAction::UpdateSpecies(_))
                | (BotAction::UpdatePlant(_), BotAction::UpdatePlant(_))
                | (BotAction::MoveToGraveyard(_), BotAction::MoveToGraveyard(_))
        )
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

impl fmt::Display for BotAction {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BotAction::Idle => frmt.write_str("Idle"),
            BotAction::WaterPlants(_) => frmt.write_str("Water Plants"),
            BotAction::WaterLocation(_) => frmt.write_str("Water Location"),
            BotAction::Rain(_) => frmt.write_str("Rain"),
            BotAction::FertilizePlants(_) => frmt.write_str("Fertilize Plants"),
            BotAction::NewGrowth(_) => frmt.write_str("New Growth"),
            BotAction::NewActivity(_) => frmt.write_str("New Activity"),
            BotAction::NewPlant(_) => frmt.write_str("New Plant"),
            BotAction::NewSpecies(_) => frmt.write_str("New Species"),
            BotAction::UpdateSpecies(_) => frmt.write_str("Update Species"),
            BotAction::UpdatePlant(_) => frmt.write_str("Update Plant"),
            BotAction::MoveToGraveyard(_) => frmt.write_str("Move To Graveyard"),
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
