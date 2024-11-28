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

impl SQLiteDB {
    pub fn new(path: PathBuf) -> Result<SQLiteDB, Error> {
        let con = sqlite::open(path.clone())?;
        Ok(SQLiteDB {
            db_path: path,
            connection: con,
        })
    }
}

impl DatabaseManager for SQLiteDB {
    // Plant Methods
    fn get_all_plants(&mut self) -> Result<Vec<Plant>, Box<dyn StdErr>> {
        /*        let info_query = "SELECT * FROM plants";
        let growth_query = "SELECT * FROM growth WHERE plant=%s";
        let activity_query = "SELECT * FROM activities WHERE plant=%s";
        let image_query = "SELECT * FROM plant_images WHERE plant_name=%s";*/
        todo!()
    }

    fn get_plants_by_location(&mut self, _location: &str) -> Result<Vec<Plant>, Box<dyn StdErr>> {
        //        let names_query = "SELECT name FROM plants WHERE location = %s";
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
        //        let query = "SELECT * FROM species;";
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
        let query = "SELECT * FROM locations";
        let mut locations = vec![];
        let location_callback = |rows: &[(&str, Option<&str>)]| {
            let mut location = "";
            let mut outside = false;
            for (key, val) in rows.into_iter() {
                match *key {
                    "name" => {
                        if let Some(name) = *val {
                            location = name
                        } else {
                            continue;
                        }
                    }
                    "outside" => outside = *val == Some("1"),
                    _ => continue,
                }
            }
            locations.push(Location {
                name: location.to_owned(),
                outside,
            });
            true
        };
        self.connection.iterate(query, location_callback)?;
        Ok(locations)
    }
    fn get_location(&mut self, location_name: &str) -> Result<Location, Box<dyn StdErr>> {
        let query = format!("SELECT * FROM locations WHERE name LIKE '%{location_name}%'");
        let mut location = None;
        let loc_callback = |rows: &[(&str, Option<&str>)]| {
            let mut name = "";
            let mut outside = false;
            for (key, val) in rows.iter() {
                match *key {
                    "name" => {
                        if let Some(loc) = *val {
                            name = loc
                        } else {
                            continue;
                        }
                    }
                    "outside" => outside = *val == Some("1"),
                    _ => continue,
                }
            }
            if name != "" {
                location = Some(Location {
                    name: name.to_owned(),
                    outside,
                });
            }
            true
        };
        self.connection.iterate(query, loc_callback)?;

        match location {
            None => Err(Box::new(Error::LocationNotFound {
                name: location_name.to_owned(),
            })),
            Some(loc) => Ok(loc),
        }
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
