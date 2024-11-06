use super::database_manager::DatabaseManager;
use plants::{
    graveyard::GraveyardPlant,
    growth_item::GrowthItem,
    location::Location,
    log_item::LogItem,
    plant::{Plant, PlantInfo},
    species::Species,
};
use sqlite::Connection;
use std::{error::Error as StdErr, path::PathBuf};

pub mod errors;
use errors::Error;

pub struct SQLiteDB {
    pub db_path: PathBuf,
    pub connection: Connection,
}

/*
Expects the following database schems

table Plants

*/
impl SQLiteDB {
    pub fn new(path: PathBuf) -> Result<SQLiteDB, Error> {
        let con = sqlite::open(path)?;
        Ok(SQLiteDB {
            db_path: path,
            connection: con,
        })
    }
}

impl DatabaseManager for SQLiteDB {
    // Plant Methods
    fn get_all_plants(&mut self) -> Result<Vec<Plant>, Box<dyn StdErr>> {
        todo!()
    }
    fn get_plants_by_location(&mut self, _location: &str) -> Result<Vec<Plant>, Box<dyn StdErr>> {
        todo!()
    }
    fn get_plant(&mut self, _plant_name: &str) -> Result<Plant, Box<dyn StdErr>> {
        todo!()
    }
    fn get_plants_species(&mut self, _species_name: &str) -> Result<Vec<Plant>, Box<dyn StdErr>> {
        todo!()
    }
    fn get_num_plants(&mut self) -> Result<i32, Box<dyn StdErr>> {
        todo!()
    }
    fn write_plant(&mut self, plant: PlantInfo) -> Result<(), Box<dyn StdErr>> {
        self.write_plants(vec![plant])
    }
    fn write_plants(&mut self, _plants: Vec<PlantInfo>) -> Result<(), Box<dyn StdErr>> {
        todo!()
    }

    // Species Methods
    fn get_all_species(&mut self) -> Result<Vec<Species>, Box<dyn StdErr>> {
        todo!()
    }
    fn get_species(&mut self, _species_name: &str) -> Result<Species, Box<dyn StdErr>> {
        todo!()
    }
    fn write_species(&mut self, _species: Species) -> Result<(), Box<dyn StdErr>> {
        todo!()
    }

    // Graveyard Methods
    fn get_graveyard(&mut self) -> Result<Vec<GraveyardPlant>, Box<dyn StdErr>> {
        todo!()
    }
    fn kill_plant(&mut self, _plant: GraveyardPlant) -> Result<(), Box<dyn StdErr>> {
        todo!()
    }

    // Location Methods
    fn get_locations(&mut self) -> Result<Vec<Location>, Box<dyn StdErr>> {
        todo!()
    }
    fn get_location(&mut self, _location_name: &str) -> Result<Location, Box<dyn StdErr>> {
        todo!()
    }

    // Log Methods
    fn get_logs(&mut self) -> Result<Vec<LogItem>, Box<dyn StdErr>> {
        todo!()
    }
    fn write_logs(&mut self, _logs: Vec<LogItem>) -> Result<(), Box<dyn StdErr>> {
        todo!()
    }
    fn write_log(&mut self, log: LogItem) -> Result<(), Box<dyn StdErr>> {
        self.write_logs(vec![log])
    }

    // Growth Methods
    fn get_growth(&mut self) -> Result<Vec<GrowthItem>, Box<dyn StdErr>> {
        todo!()
    }
    fn write_growths(&mut self, _growth: Vec<GrowthItem>) -> Result<(), Box<dyn StdErr>> {
        todo!()
    }
    fn write_growth(&mut self, growth: GrowthItem) -> Result<(), Box<dyn StdErr>> {
        self.write_growths(vec![growth])
    }

    // Existence Methods
    fn plant_exists(&mut self, _plant_name: &str) -> Result<bool, Box<dyn StdErr>> {
        todo!()
    }
    fn species_exists(&mut self, _species_name: &str) -> Result<bool, Box<dyn StdErr>> {
        todo!()
    }
}
