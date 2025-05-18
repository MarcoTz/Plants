mod fertilize_plants;
mod get_location_plants;
mod get_plant_activities;
mod get_plant_details;
mod get_plant_fertilizing;
mod get_plant_growth;
mod get_plant_watering;
mod get_species_details;
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
pub use get_location_plants::GetLocationPlants;
pub use get_plant_activities::GetPlantActivities;
pub use get_plant_details::GetPlantDetails;
pub use get_plant_fertilizing::GetPlantFertilizing;
pub use get_plant_growth::GetPlantGrowth;
pub use get_plant_watering::GetPlantWatering;
pub use get_species_details::GetSpeciesDetails;
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

#[derive(Clone, Debug)]
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
    GetLocationPlants(GetLocationPlants),
    GetPlantDetails(GetPlantDetails),
    GetSpeciesDetails(GetSpeciesDetails),
    GetPlantActivities(GetPlantActivities),
    GetPlantWatering(GetPlantWatering),
    GetPlantFertilizing(GetPlantFertilizing),
    GetPlantGrowth(GetPlantGrowth),
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
                | (
                    BotAction::GetLocationPlants(_),
                    BotAction::GetLocationPlants(_)
                )
                | (BotAction::GetPlantDetails(_), BotAction::GetPlantDetails(_))
                | (
                    BotAction::GetSpeciesDetails(_),
                    BotAction::GetSpeciesDetails(_)
                )
                | (
                    BotAction::GetPlantActivities(_),
                    BotAction::GetPlantActivities(_)
                )
                | (
                    BotAction::GetPlantWatering(_),
                    BotAction::GetPlantWatering(_)
                )
                | (
                    BotAction::GetPlantFertilizing(_),
                    BotAction::GetPlantFertilizing(_)
                )
                | (BotAction::GetPlantGrowth(_), BotAction::GetPlantGrowth(_))
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BotAction::Idle => f.write_str("Idle"),
            BotAction::WaterPlants(_) => f.write_str("Water Plants"),
            BotAction::WaterLocation(_) => f.write_str("Water Location"),
            BotAction::Rain(_) => f.write_str("Rain"),
            BotAction::FertilizePlants(_) => f.write_str("Fertilize Plants"),
            BotAction::NewGrowth(_) => f.write_str("New Growth"),
            BotAction::NewActivity(_) => f.write_str("New Activity"),
            BotAction::NewPlant(_) => f.write_str("New Plant"),
            BotAction::NewSpecies(_) => f.write_str("New Species"),
            BotAction::UpdateSpecies(_) => f.write_str("Update Species"),
            BotAction::UpdatePlant(_) => f.write_str("Update Plant"),
            BotAction::MoveToGraveyard(_) => f.write_str("Move To Graveyard"),
            BotAction::GetLocationPlants(_) => f.write_str("Get Plants at Location"),
            BotAction::GetPlantDetails(_) => f.write_str("Get Plant Details"),
            BotAction::GetSpeciesDetails(_) => f.write_str("Get Species Details"),
            BotAction::GetPlantActivities(_) => f.write_str("Get Activities for Plant"),
            BotAction::GetPlantWatering(_) => f.write_str("Get Watering Dates for Plant"),
            BotAction::GetPlantFertilizing(_) => f.write_str("Get Fertilizing Dates for Plant"),
            BotAction::GetPlantGrowth(_) => f.write_str("Get Growth Updates for Plant"),
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
            BotAction::GetLocationPlants(glp) => glp.handle_input(input, db_man),
            BotAction::GetPlantDetails(gpd) => gpd.handle_input(input, db_man),
            BotAction::GetSpeciesDetails(gsd) => gsd.handle_input(input, db_man),
            BotAction::GetPlantActivities(gpa) => gpa.handle_input(input, db_man),
            BotAction::GetPlantWatering(gpw) => gpw.handle_input(input, db_man),
            BotAction::GetPlantFertilizing(gpf) => gpf.handle_input(input, db_man),
            BotAction::GetPlantGrowth(gpg) => gpg.handle_input(input, db_man),
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
            BotAction::GetLocationPlants(glp) => glp.is_done(),
            BotAction::GetPlantDetails(gpd) => gpd.is_done(),
            BotAction::GetSpeciesDetails(gsd) => gsd.is_done(),
            BotAction::GetPlantActivities(gpa) => gpa.is_done(),
            BotAction::GetPlantWatering(gpw) => gpw.is_done(),
            BotAction::GetPlantFertilizing(gpf) => gpf.is_done(),
            BotAction::GetPlantGrowth(gpg) => gpg.is_done(),
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
            BotAction::GetLocationPlants(glp) => glp.write_result(db_man),
            BotAction::GetPlantDetails(gpd) => gpd.write_result(db_man),
            BotAction::GetSpeciesDetails(gsd) => gsd.write_result(db_man),
            BotAction::GetPlantActivities(gpa) => gpa.write_result(db_man),
            BotAction::GetPlantWatering(gpw) => gpw.write_result(db_man),
            BotAction::GetPlantFertilizing(gpf) => gpf.write_result(db_man),
            BotAction::GetPlantGrowth(gpg) => gpg.write_result(db_man),
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
            BotAction::GetLocationPlants(glp) => glp.get_next_prompt(),
            BotAction::GetPlantDetails(gpd) => gpd.get_next_prompt(),
            BotAction::GetSpeciesDetails(gsd) => gsd.get_next_prompt(),
            BotAction::GetPlantActivities(gpa) => gpa.get_next_prompt(),
            BotAction::GetPlantWatering(gpw) => gpw.get_next_prompt(),
            BotAction::GetPlantFertilizing(gpf) => gpf.get_next_prompt(),
            BotAction::GetPlantGrowth(gpg) => gpg.get_next_prompt(),
        }
    }
}

#[cfg(test)]
mod bot_action_tests {
    use super::{
        Action, BotAction, FertilizePlants, MoveToGraveyard, NewActivity, NewGrowth, NewPlant,
        NewSpecies, Rain, UpdatePlant, UpdateSpecies, WaterLocation, WaterPlants,
    };
    use crate::test_common::DummyManager;

    #[test]
    fn action_eq() {
        assert_eq!(BotAction::Idle, BotAction::Idle)
    }

    #[test]
    fn action_neq() {
        assert!(BotAction::Idle != BotAction::Rain(Rain {}))
    }

    #[test]
    fn display_idle() {
        let result = format!("{}", BotAction::Idle);
        let expected = "Idle";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_water() {
        let result = format!("{}", BotAction::WaterPlants(WaterPlants::default()));
        let expected = "Water Plants";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_waterloc() {
        let result = format!("{}", BotAction::WaterLocation(WaterLocation::default()));
        let expected = "Water Location";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_rain() {
        let result = format!("{}", BotAction::Rain(Rain {}));
        let expected = "Rain";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_fertilize() {
        let result = format!("{}", BotAction::FertilizePlants(FertilizePlants::default()));
        let expected = "Fertilize Plants";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_newgrowth() {
        let result = format!("{}", BotAction::NewGrowth(NewGrowth::default()));
        let expected = "New Growth";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_newactivity() {
        let result = format!("{}", BotAction::NewActivity(NewActivity::default()));
        let expected = "New Activity";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_newplant() {
        let result = format!("{}", BotAction::NewPlant(NewPlant::default()));
        let expected = "New Plant";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_newspecies() {
        let result = format!("{}", BotAction::NewSpecies(NewSpecies::default()));
        let expected = "New Species";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_updatespecies() {
        let result = format!("{}", BotAction::UpdateSpecies(UpdateSpecies::default()));
        let expected = "Update Species";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_updateplant() {
        let result = format!("{}", BotAction::UpdatePlant(UpdatePlant::default()));
        let expected = "Update Plant";
        assert_eq!(result, expected)
    }

    #[test]
    fn displat_movegraveyard() {
        let result = format!("{}", BotAction::MoveToGraveyard(MoveToGraveyard::default()));
        let expected = "Move To Graveyard";
        assert_eq!(result, expected)
    }

    #[test]
    fn input_idle() {
        let result = BotAction::Idle.handle_input("".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_water() {
        let result = BotAction::WaterPlants(WaterPlants::default())
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = WaterPlants::default()
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn input_waterloc() {
        let result = BotAction::WaterLocation(WaterLocation::default())
            .handle_input("Inside".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = WaterLocation::default()
            .handle_input("Inside".to_owned(), &mut DummyManager {})
            .unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn input_rain() {
        let result = BotAction::Rain(Rain {}).handle_input("".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_fertilize() {
        let result = BotAction::FertilizePlants(FertilizePlants::default())
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = FertilizePlants::default()
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn input_newgrowth() {
        let result = BotAction::NewGrowth(NewGrowth::default())
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = NewGrowth::default()
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn input_newspecies() {
        let result = BotAction::NewSpecies(NewSpecies::default())
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = NewSpecies::default()
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn input_newplant() {
        let result = BotAction::NewPlant(NewPlant::default())
            .handle_input("newplant".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = NewPlant::default()
            .handle_input("newplant".to_owned(), &mut DummyManager {})
            .unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn input_newactivity() {
        let result = BotAction::NewActivity(NewActivity::default())
            .handle_input("01.01.1970".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = NewActivity::default()
            .handle_input("01.01.1970".to_owned(), &mut DummyManager {})
            .unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn input_updateplant() {
        let result = BotAction::UpdatePlant(UpdatePlant::default())
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = UpdatePlant::default()
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn input_udpatespecies() {
        let result = BotAction::UpdateSpecies(UpdateSpecies::default())
            .handle_input("Species1".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = UpdateSpecies::default()
            .handle_input("Species1".to_owned(), &mut DummyManager {})
            .unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn input_movegraveyard() {
        let result = BotAction::MoveToGraveyard(MoveToGraveyard::default())
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = MoveToGraveyard::default()
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn done_idle() {
        let result = BotAction::Idle.is_done();
        assert!(result)
    }

    #[test]
    fn done_water() {
        let result = BotAction::WaterPlants(WaterPlants::default()).is_done();
        let expected = WaterPlants::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn done_waterloc() {
        let result = BotAction::WaterLocation(WaterLocation::default()).is_done();
        let expected = WaterLocation::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn done_rain() {
        let result = BotAction::Rain(Rain::default()).is_done();
        let expected = Rain::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn done_fertilize() {
        let result = BotAction::FertilizePlants(FertilizePlants::default()).is_done();
        let expected = FertilizePlants::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn done_newgrowth() {
        let result = BotAction::NewGrowth(NewGrowth::default()).is_done();
        let expected = NewGrowth::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn done_newactivity() {
        let result = BotAction::NewActivity(NewActivity::default()).is_done();
        let expected = NewGrowth::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn done_newplant() {
        let result = BotAction::NewPlant(NewPlant::default()).is_done();
        let expected = NewPlant::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn doen_newspecies() {
        let result = BotAction::NewSpecies(NewSpecies::default()).is_done();
        let expected = NewSpecies::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn done_updatespecies() {
        let result = BotAction::UpdateSpecies(UpdateSpecies::default()).is_done();
        let expected = UpdateSpecies::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn done_updateplant() {
        let result = BotAction::UpdatePlant(UpdatePlant::default()).is_done();
        let expected = UpdatePlant::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn done_movegraveyard() {
        let result = BotAction::MoveToGraveyard(MoveToGraveyard::default()).is_done();
        let expected = MoveToGraveyard::default().is_done();
        assert_eq!(result, expected)
    }

    #[test]
    fn write_idle() {
        let result = BotAction::Idle.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_water() {
        let result =
            BotAction::WaterPlants(WaterPlants::default()).write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_waterloc() {
        let result =
            BotAction::WaterLocation(WaterLocation::default()).write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_rain() {
        let result = BotAction::Rain(Rain::default())
            .write_result(&mut DummyManager {})
            .unwrap();
        let expected = Rain::default().write_result(&mut DummyManager {}).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn write_fertilize() {
        let result = BotAction::FertilizePlants(FertilizePlants::default())
            .write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_newgrowth() {
        let result = BotAction::NewGrowth(NewGrowth::default()).write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_newactivity() {
        let result =
            BotAction::NewActivity(NewActivity::default()).write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_newplant() {
        let result = BotAction::NewPlant(NewPlant::default()).write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_newspecies() {
        let result =
            BotAction::NewSpecies(NewSpecies::default()).write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_updateplant() {
        let result =
            BotAction::UpdatePlant(UpdatePlant::default()).write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_updatespecies() {
        let result =
            BotAction::UpdateSpecies(UpdateSpecies::default()).write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_movegraveyard() {
        let result = BotAction::MoveToGraveyard(MoveToGraveyard::default())
            .write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn get_next_idle() {
        let result = BotAction::Idle.get_next_prompt();
        assert!(result.is_err())
    }

    #[test]
    fn get_next_water() {
        let result = BotAction::WaterPlants(WaterPlants::default())
            .get_next_prompt()
            .unwrap();
        let expected = WaterPlants::default().get_next_prompt().unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn get_next_waterloc() {
        let result = BotAction::WaterLocation(WaterLocation::default())
            .get_next_prompt()
            .unwrap();
        let expected = WaterLocation::default().get_next_prompt().unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn get_next_rain() {
        let result = BotAction::Rain(Rain::default()).get_next_prompt();
        assert!(result.is_err())
    }

    #[test]
    fn get_next_fertilize() {
        let result = BotAction::FertilizePlants(FertilizePlants::default())
            .get_next_prompt()
            .unwrap();
        let expected = FertilizePlants::default().get_next_prompt().unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn get_next_newgrowth() {
        let result = BotAction::NewGrowth(NewGrowth::default())
            .get_next_prompt()
            .unwrap();
        let expected = NewGrowth::default().get_next_prompt().unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn get_next_newactivity() {
        let result = BotAction::NewActivity(NewActivity::default())
            .get_next_prompt()
            .unwrap();
        let expected = NewActivity::default().get_next_prompt().unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn get_next_newplant() {
        let result = BotAction::NewPlant(NewPlant::default())
            .get_next_prompt()
            .unwrap();
        let expected = NewPlant::default().get_next_prompt().unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn get_next_newspecies() {
        let result = BotAction::NewSpecies(NewSpecies::default())
            .get_next_prompt()
            .unwrap();
        let expected = NewSpecies::default().get_next_prompt().unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn get_next_updatespecies() {
        let result = BotAction::UpdateSpecies(UpdateSpecies::default())
            .get_next_prompt()
            .unwrap();
        let expected = UpdateSpecies::default().get_next_prompt().unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn get_next_updateplant() {
        let result = BotAction::UpdatePlant(UpdatePlant::default())
            .get_next_prompt()
            .unwrap();
        let expected = UpdatePlant::default().get_next_prompt().unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn get_next_movegraveyard() {
        let result = BotAction::MoveToGraveyard(MoveToGraveyard::default())
            .get_next_prompt()
            .unwrap();
        let expected = MoveToGraveyard::default().get_next_prompt().unwrap();
        assert_eq!(result, expected)
    }
}
