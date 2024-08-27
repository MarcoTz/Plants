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
use fertilize_plants::FertilizePlants;
use move_to_graveyard::MoveToGraveyard;
use new_activity::NewActivity;
use new_growth::NewGrowth;
use new_plant::NewPlant;
use new_species::NewSpecies;
use rain::Rain;
use update_plant::UpdatePlant;
use update_species::UpdateSpecies;
use water_location::WaterLocation;
use water_plants::WaterPlants;

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
