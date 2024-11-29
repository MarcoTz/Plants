use super::database_manager::DatabaseManager;
use chrono::NaiveDate;
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
    pub date_format: String,
}

impl SQLiteDB {
    pub fn new(path: PathBuf) -> Result<SQLiteDB, Error> {
        let con = sqlite::open(path.clone())?;
        Ok(SQLiteDB {
            db_path: path,
            connection: con,
            date_format: "%d.%m.%Y".to_owned(),
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
        let query = "SELECT * FROM activities";
        let mut logs = vec![];
        let log_callback = |rows: &[(&str, Option<&str>)]| {
            let mut name = "";
            let mut date = None;
            let mut plant = "";
            let mut note = None;
            for (key, value) in rows.iter() {
                let val = if let Some(val) = value {
                    *val
                } else {
                    continue;
                };

                match *key {
                    "name" => name = val,
                    "date" => {
                        date = Some(NaiveDate::parse_from_str(val, &self.date_format).unwrap())
                    }
                    "plant" => plant = val,
                    "note" => {
                        note = if val != "" {
                            Some(val.to_owned())
                        } else {
                            None
                        }
                    }
                    _ => continue,
                }
            }
            if name != "" && plant != "" && date.is_some() {
                logs.push(LogItem {
                    activity: name.to_owned(),
                    date: date.unwrap(),
                    plant: plant.to_owned(),
                    note,
                });
                true
            } else {
                false
            }
        };
        self.connection.iterate(query, log_callback)?;
        Ok(logs)
    }

    fn write_logs(&mut self, _logs: Vec<LogItem>) -> Result<(), Box<dyn StdErr>> {
        todo!()
    }

    // Growth Methods
    fn get_growth(&mut self) -> Result<Vec<GrowthItem>, Box<dyn StdErr>> {
        let query = "SELECT * FROM growth";
        let mut growth = vec![];
        let growth_callback = |rows: &[(&str, Option<&str>)]| {
            let mut plant = "";
            let mut date = None;
            let mut height = -1.0;
            let mut width = -1.0;
            let mut note = None;
            let mut health = -1;
            for (key, value) in rows.iter() {
                let val = if let Some(val) = value {
                    *val
                } else {
                    continue;
                };

                match *key {
                    "plant" => plant = val,
                    "date" => {
                        date = Some(NaiveDate::parse_from_str(val, &self.date_format).unwrap())
                    }
                    "height_cm" => height = val.parse::<f32>().unwrap(),
                    "width_cm" => width = val.parse::<f32>().unwrap(),
                    "note" => {
                        note = if val != "" {
                            Some(val.to_owned())
                        } else {
                            None
                        }
                    }
                    "health" => health = val.parse::<i32>().unwrap(),
                    _ => continue,
                }
            }
            if plant != "" && date.is_some() && height != -1.0 && width != -1.0 && health != -1 {
                growth.push(GrowthItem {
                    plant: plant.to_owned(),
                    date: date.unwrap(),
                    height_cm: height,
                    width_cm: width,
                    note,
                    health,
                });
            }
            true
        };
        self.connection.iterate(query, growth_callback)?;
        Ok(growth)
    }

    fn write_growths(&mut self, growth: Vec<GrowthItem>) -> Result<(), Box<dyn StdErr>> {
        if growth.is_empty() {
            return Ok(());
        }

        let mut insert_strs = vec![];
        for item in growth.into_iter() {
            let note_str = if let Some(note) = item.note {
                format!("'{note}'")
            } else {
                "null".to_owned()
            };

            insert_strs.push(format!(
                "('{}','{}',{},{},{},{})",
                item.plant,
                item.date.format(&self.date_format),
                item.height_cm,
                item.width_cm,
                note_str,
                item.health
            ));
        }

        let query = format!(
            "INSERT INTO growth (plant,date,height_cm,width_cm,note,health) VALUES {};",
            insert_strs.join(", ")
        );
        self.connection.execute(query)?;
        Ok(())
    }

    // Existence Methods
    fn plant_exists(&mut self, plant_name: &str) -> Result<bool, Box<dyn StdErr>> {
        let query = format!("SELECT COUNT(*) AS num FROM plants WHERE name LIKE '%{plant_name}%'");
        let mut exists = false;
        let count_callback = |rows: &[(&str, Option<&str>)]| {
            for (key, val) in rows.iter() {
                if *key == "num" && val.is_some() {
                    let num = val.unwrap().parse::<usize>().unwrap();
                    exists = num > 0;
                    return true;
                }
            }
            true
        };
        self.connection.iterate(query, count_callback)?;
        Ok(exists)
    }
    fn species_exists(&mut self, species_name: &str) -> Result<bool, Box<dyn StdErr>> {
        let query =
            format!("SELECT COUNT(*) AS num FROM species WHERE name LIKE '%{species_name}%'");
        let mut exists = false;
        let count_callback = |rows: &[(&str, Option<&str>)]| {
            for (key, value) in rows.iter() {
                if *key == "num" && value.is_some() {
                    let num = value.unwrap().parse::<usize>().unwrap();
                    exists = num > 0;
                    return true;
                }
            }
            true
        };
        self.connection.iterate(query, count_callback)?;
        Ok(exists)
    }
}
