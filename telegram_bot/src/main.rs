pub mod action_handler;
pub mod bot_actions;
pub mod commands;
pub mod config;
pub mod errors;

use action_handler::ActionHandler;
use bot_api::bot::Bot;
use config::load_config;
use database::file_backend::FileDB;
use errors::Error;
use log::Level;
use logger::{file_logger::FileLogger, init::init_logger};

static LOGGER: FileLogger = FileLogger {
    level: Level::Info,
    file_path: "bot.log",
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_logger(&LOGGER).map_err(|_| Error::Logger)?;
    log::info!("Succesfully set up logger");

    let conf = load_config()?;
    log::info!("Successfully loaded config");

    let mut bot = Bot::new(conf.api_key);
    let mut handler = ActionHandler::new(conf.white_list, FileDB::default());

    log::info!("Running bot");
    bot.run(&mut handler).await;
    Ok(())
}

#[cfg(test)]
pub mod test_common {
    use chrono::NaiveDate;
    use database::database_manager::DatabaseManager;
    use plants::{
        graveyard::GraveyardPlant,
        growth_item::GrowthItem,
        location::Location,
        log_item::LogItem,
        plant::{Plant, PlantImage, PlantInfo, PlantLocation, PlantSpecies},
        species::{Species, SunlightRequirement},
    };
    use std::{error::Error, fmt, path::PathBuf};

    pub struct DummyManager;
    #[derive(Debug)]
    struct DummyErr;
    impl fmt::Display for DummyErr {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            panic!("not implemented")
        }
    }
    impl std::error::Error for DummyErr {}

    impl DatabaseManager for DummyManager {
        fn get_all_plants(&mut self) -> Result<Vec<Plant>, Box<dyn Error>> {
            Ok(vec![])
        }

        fn get_plants_by_location(&mut self, name: &str) -> Result<Vec<Plant>, Box<dyn Error>> {
            if name == "Inside" {
                Ok(vec![example_plant()])
            } else {
                Err(Box::new(DummyErr))
            }
        }

        fn get_plant(&mut self, _: &str) -> Result<Plant, Box<dyn Error>> {
            Ok(example_plant())
        }

        fn get_plants_species(&mut self, _: &str) -> Result<Vec<Plant>, Box<dyn Error>> {
            Ok(vec![])
        }

        fn get_num_plants(&mut self) -> Result<i32, Box<dyn Error>> {
            Ok(self.get_all_plants().unwrap().len() as i32)
        }

        fn write_plants(&mut self, _: Vec<PlantInfo>) -> Result<(), Box<dyn Error>> {
            Ok(())
        }

        fn get_all_species(&mut self) -> Result<Vec<Species>, Box<dyn Error>> {
            Ok(vec![])
        }

        fn get_species(&mut self, name: &str) -> Result<Species, Box<dyn Error>> {
            if name == "Species1" {
                Ok(example_species())
            } else {
                Err(Box::new(DummyErr {}))
            }
        }

        fn write_species(&mut self, _: Species) -> Result<(), Box<dyn Error>> {
            Ok(())
        }

        fn get_graveyard(&mut self) -> Result<Vec<GraveyardPlant>, Box<dyn Error>> {
            Ok(vec![])
        }

        fn kill_plant(&mut self, _: GraveyardPlant) -> Result<(), Box<dyn Error>> {
            Ok(())
        }

        fn get_locations(&mut self) -> Result<Vec<Location>, Box<dyn Error>> {
            Ok(vec![example_location()])
        }

        fn get_location(&mut self, name: &str) -> Result<Location, Box<dyn Error>> {
            if name == "Inside" {
                Ok(example_location())
            } else {
                Err(Box::new(DummyErr {}))
            }
        }

        fn get_logs(&mut self) -> Result<Vec<LogItem>, Box<dyn Error>> {
            Ok(vec![])
        }

        fn write_logs(&mut self, _: Vec<LogItem>) -> Result<(), Box<dyn Error>> {
            Ok(())
        }

        fn get_growth(&mut self) -> Result<Vec<GrowthItem>, Box<dyn Error>> {
            Ok(vec![])
        }

        fn write_growths(&mut self, _: Vec<GrowthItem>) -> Result<(), Box<dyn Error>> {
            Ok(())
        }

        fn plant_exists(&mut self, name: &str) -> Result<bool, Box<dyn Error>> {
            Ok(vec!["Plant1", "Plant2", "Plant3"].contains(&name))
        }
        fn species_exists(&mut self, name: &str) -> Result<bool, Box<dyn Error>> {
            Ok(vec!["Species1", "Species2", "Species3"].contains(&name))
        }
    }
    pub fn example_species() -> Species {
        Species {
            name: "Test species".to_owned(),
            scientific_name: "Scientific Name".to_owned(),
            genus: "Genus".to_owned(),
            family: "Family".to_owned(),
            sunlight: SunlightRequirement::Direct,
            temp_min: 0.0,
            temp_max: 30.0,
            opt_temp_min: 10.0,
            opt_temp_max: 25.0,
            planting_distance: Some(30.0),
            ph_min: 4.5,
            ph_max: 8.5,
            watering_notes: vec![],
            avg_watering_days: Some(7),
            fertilizing_notes: vec![],
            avg_fertilizing_days: Some(14),
            pruning_notes: vec![],
            companions: vec![],
            additional_notes: vec![],
        }
    }

    pub fn example_plant() -> Plant {
        Plant {
            info: example_plant_info(),
            growth: vec![example_growth1(), example_growth2()],
            activities: vec![example_activity1(), example_activity2()],
            images: vec![example_image1(), example_image2()],
        }
    }
    pub fn example_plant_info() -> PlantInfo {
        PlantInfo {
            name: "A Plant".to_owned(),
            species: PlantSpecies::Species(Box::new(example_species())),
            location: PlantLocation::Location(Box::new(example_location())),
            origin: "An Origin".to_owned(),
            obtained: example_date1(),
            auto_water: false,
            notes: vec![],
        }
    }
    pub fn example_growth1() -> GrowthItem {
        GrowthItem {
            plant: "A Plant".to_owned(),
            date: example_date1(),
            height_cm: 10.0,
            width_cm: 10.0,
            note: None,
            health: 3,
        }
    }

    pub fn example_growth2() -> GrowthItem {
        GrowthItem {
            plant: "A Plant".to_owned(),
            date: example_date2(),
            height_cm: 15.0,
            width_cm: 15.0,
            note: None,
            health: 4,
        }
    }

    pub fn example_activity1() -> LogItem {
        LogItem {
            activity: "Watering".to_owned(),
            date: example_date1(),
            plant: "A Plant".to_owned(),
            note: None,
        }
    }

    pub fn example_activity2() -> LogItem {
        LogItem {
            activity: "Fertilizing".to_owned(),
            date: example_date1(),
            plant: "A Plant".to_owned(),
            note: None,
        }
    }

    pub fn example_image1() -> PlantImage {
        PlantImage {
            created: example_date1(),
            file_name: "01011970.jpg".to_owned(),
            file_path: PathBuf::from("./"),
        }
    }

    pub fn example_image2() -> PlantImage {
        PlantImage {
            created: example_date2(),
            file_name: "02011970.jpg".to_owned(),
            file_path: PathBuf::from("/"),
        }
    }

    pub fn example_location() -> Location {
        Location {
            name: "Inside".to_owned(),
            outside: false,
        }
    }

    pub fn example_date1() -> NaiveDate {
        NaiveDate::parse_from_str("01.01.1970", "%d.%m.%Y").unwrap()
    }

    pub fn example_date2() -> NaiveDate {
        NaiveDate::parse_from_str("02.01.1970", "%d.%m.%Y").unwrap()
    }
}
