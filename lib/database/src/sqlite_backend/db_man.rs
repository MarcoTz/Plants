use super::{errors::Error, SQLiteDB};
use crate::{database_manager::DatabaseManager, file_backend::json_to_plant::load_images};
use plants::{
    graveyard::GraveyardPlant,
    growth_item::GrowthItem,
    location::Location,
    log_item::LogItem,
    plant::{Plant, PlantInfo},
    species::Species,
};

use std::{error::Error as StdErr, path::PathBuf};

impl DatabaseManager for SQLiteDB {
    // Plant Methods
    fn get_all_plants(&mut self) -> Result<Vec<Plant>, Box<dyn StdErr>> {
        let info_query = "SELECT * FROM plants";
        let info_maps = self.read_rows(
            info_query,
            vec![
                "name",
                "species",
                "location",
                "origin",
                "obtained",
                "auto_water",
                "notes",
            ],
        )?;
        let mut infos = vec![];
        for mut map in info_maps.into_iter() {
            map.insert("date_format".to_owned(), self.date_format.clone());
            let info: PlantInfo = map.try_into()?;
            infos.push(info);
        }

        let mut plants = vec![];
        for plant in infos.into_iter() {
            let growths = self.get_growth_plant(&plant.name)?;
            let logs = self.get_logs_plant(&plant.name)?;

            let img_dir = self.plants_dir.join(plant.name.replace(' ', ""));
            let images = load_images(&img_dir)?;
            plants.push(Plant {
                info: plant,
                growth: growths,
                activities: logs,
                images,
            });
        }
        Ok(plants)
    }

    fn get_plants_by_location(&mut self, location: &str) -> Result<Vec<Plant>, Box<dyn StdErr>> {
        let location_query =
            format!("SELECT UNIQUE name FROM plants WHERE location LIKE '%{location}%'");
        let name_maps = self.read_rows(&location_query, vec!["name"])?;
        let names = name_maps
            .into_iter()
            .map(|map| {
                map.get("name").cloned().ok_or(Error::MissingValue {
                    key: "name".to_owned(),
                })
            })
            .collect::<Result<Vec<String>, Error>>()?;

        let mut plants = vec![];
        for name in names {
            plants.push(self.get_plant(&name)?);
        }
        Ok(plants)
    }

    fn get_plant(&mut self, plant_name: &str) -> Result<Plant, Box<dyn StdErr>> {
        let info_query = format!("SELECT * FROM plants WHERE name={plant_name}");
        let info_maps = self.read_rows(
            &info_query,
            vec![
                "name",
                "species",
                "location",
                "origin",
                "obtained",
                "auto_water",
                "notes",
            ],
        )?;
        let mut plant_map = info_maps.first().cloned().ok_or(Error::PlantNotFound {
            name: plant_name.to_owned(),
        })?;
        plant_map.insert("date_format".to_owned(), self.date_format.clone());
        let info: PlantInfo = plant_map.try_into()?;
        let growths = self.get_growth_plant(&plant_name)?;
        let logs = self.get_logs_plant(&plant_name)?;
        let img_dir = self.plants_dir.join(plant_name.replace(' ', ""));
        let images = load_images(&img_dir)?;
        Ok(Plant {
            info,
            growth: growths,
            activities: logs,
            images,
        })
    }

    fn get_plants_species(&mut self, species_name: &str) -> Result<Vec<Plant>, Box<dyn StdErr>> {
        let species_query = format!("SELECT name FROM plants WHERE species={species_name}");
        let species_maps = self.read_rows(&species_query, vec!["name"])?;
        let names: Vec<String> = species_maps
            .into_iter()
            .map(|map| {
                map.get("name")
                    .ok_or(Error::MissingValue {
                        key: "name".to_owned(),
                    })
                    .cloned()
            })
            .collect::<Result<Vec<String>, Error>>()?;

        let mut plants = vec![];
        for name in names {
            plants.push(self.get_plant(&name)?);
        }
        Ok(plants)
    }

    fn get_num_plants(&mut self) -> Result<i32, Box<dyn StdErr>> {
        let num_query = "SELECT COUNT(*) AS num FROM plants";
        let num_map = self.read_rows(num_query, vec!["num"])?;
        let fst = num_map.first().ok_or(Error::MissingValue {
            key: "num".to_owned(),
        })?;
        let num_str = fst.get("num").ok_or(Error::MissingValue {
            key: "num".to_owned(),
        })?;
        let num = num_str.parse::<i32>()?;
        Ok(num)
    }
    fn write_plants(&mut self, plants: Vec<PlantInfo>) -> Result<(), Box<dyn StdErr>> {
        let mut plant_query =
            "INSERT INTO plants (name,species,location,origin,obtained,auto_water,notes) VALUES "
                .to_owned();
        let fmt_plant = |info: PlantInfo| {
            let notes_str = if info.notes.is_empty() {
                "null".to_owned()
            } else {
                format!("'{}'", self.sanitize(&info.notes.join(", ")))
            };

            format!(
                "('{}','{}', '{}','{}','{}','{}',{})",
                self.sanitize(&info.name),
                self.sanitize(&info.species),
                self.sanitize(&info.location),
                self.sanitize(&info.origin),
                self.sanitize(&info.obtained),
                self.sanitize(&info.auto_water),
                notes_str
            )
        };
        let mut plant_strs = vec![];
        for plant in plants {
            plant_strs.push(fmt_plant(plant));
        }
        plant_query += &plant_strs.join(", ");
        plant_query += ";";
        self.connection.execute(plant_query)?;
        Ok(())
    }

    // Species Methods
    fn get_all_species(&mut self) -> Result<Vec<Species>, Box<dyn StdErr>> {
        let species_query = "SELECT * FROM species;";
        let species_maps = self.read_rows(
            species_query,
            vec![
                "name",
                "scientific_name",
                "genus",
                "family",
                "sunlight",
                "temp_min",
                "temp_max",
                "temp_min_opt",
                "temp_max_opt",
                "planting_distance",
                "ph_min",
                "ph_max",
                "watering_notes",
                "fertilizing_notes",
                "avg_watering_days",
                "avg_fertilizing_days",
                "pruning_notes",
                "companions",
                "additional_notes",
            ],
        )?;
        let mut species = vec![];
        for mut species_map in species_maps.into_iter() {
            species_map.insert("date_format".to_owned(), self.date_format.clone());
            species.push(species_map.try_into()?);
        }

        Ok(species)
    }

    fn get_species(&mut self, species_name: &str) -> Result<Species, Box<dyn StdErr>> {
        let species_query = format!("SELECT * FROM species WHERE name={species_name};");
        let species_map = self.read_rows(
            &species_query,
            vec![
                "name",
                "scientific_name",
                "genus",
                "family",
                "sunlight",
                "temp_min",
                "temp_max",
                "temp_min_opt",
                "temp_max_opt",
                "planting_distance",
                "ph_min",
                "ph_max",
                "watering_notes",
                "fertilizing_notes",
                "avg_watering_days",
                "avg_fertilizing_days",
                "pruning_notes",
                "companions",
                "additional_notes",
            ],
        )?;
        let map_fst = species_map.first().cloned().ok_or(Error::SpeciesNotFound {
            name: species_name.to_owned(),
        })?;
        let species: Species = map_fst.try_into()?;

        Ok(species)
    }

    fn write_species(&mut self, species: Species) -> Result<(), Box<dyn StdErr>> {
        let plant_distance_str = species
            .planting_distance
            .map(|f| format!("{}", f))
            .unwrap_or("null".to_owned());
        let avg_watering_days_str = species
            .avg_watering_days
            .map(|f| format!("{}", f))
            .unwrap_or("null".to_owned());
        let avg_fertilizing_str = species
            .avg_fertilizing_days
            .map(|f| format!("{}", f))
            .unwrap_or("null".to_owned());

        let watering_notes_str = if species.watering_notes.is_empty() {
            "null".to_owned()
        } else {
            format!("'{}'", self.sanitize(&species.watering_notes.join(", ")))
        };
        let fertilizing_notes_str = if species.fertilizing_notes.is_empty() {
            "null".to_owned()
        } else {
            format!("'{}'", self.sanitize(&species.fertilizing_notes.join(", ")))
        };
        let pruning_str = if species.pruning_notes.is_empty() {
            "null".to_owned()
        } else {
            format!("'{}'", self.sanitize(&species.pruning_notes.join(", ")))
        };
        let companions_str = if species.companions.is_empty() {
            "null".to_owned()
        } else {
            format!("'{}'", self.sanitize(&species.companions.join(", ")))
        };
        let notes_str = if species.additional_notes.is_empty() {
            "null".to_owned()
        } else {
            format!("'{}'", self.sanitize(&species.additional_notes.join(", ")))
        };
        let species_query = format!(
            "INSERT INTO species
            (name,
            scientific_name,
            genus,
            family,
            sunlight,
            temp_min,
            temp_max,
            temp_min_opt,
            temp_max_opt,
            planting_distance,
            ph_min,
            ph_max,
            watering_notes,
            fertilizing_notes,
            avg_watering_days,
            avg_fertilizing_days,
            pruning_notes,
            companions,
            additional_notes)
            VALUES 
            ('{}','{}','{}','{}','{}',{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
            self.sanitize(&species.name),
            self.sanitize(&species.scientific_name),
            self.sanitize(&species.genus),
            self.sanitize(&species.family),
            self.sanitize(&species.sunlight),
            species.temp_min,
            species.temp_max,
            species.opt_temp_min,
            species.opt_temp_max,
            plant_distance_str,
            species.ph_min,
            species.ph_max,
            watering_notes_str,
            fertilizing_notes_str,
            avg_watering_days_str,
            avg_fertilizing_str,
            pruning_str,
            companions_str,
            notes_str
        );
        self.connection.execute(species_query)?;
        Ok(())
    }

    // Graveyard Methods
    fn get_graveyard(&mut self) -> Result<Vec<GraveyardPlant>, Box<dyn StdErr>> {
        let query = "SELECT * FROM graveyard";
        let graveyard_maps =
            self.read_rows(&query, vec!["name", "species", "planted", "died", "reason"])?;
        let mut graveyard = vec![];
        for mut map in graveyard_maps.into_iter() {
            map.insert("date_format".to_owned(), self.date_format.clone());
            let plant: GraveyardPlant = map.try_into()?;
            graveyard.push(plant);
        }

        Ok(graveyard)
    }

    fn kill_plant(&mut self, plant: GraveyardPlant) -> Result<(), Box<dyn StdErr>> {
        let graveyard_query = format!(
            "INSERT INTO graveyard (name,species,planted,died,reason) values ('{}','{}','{}','{}','{}')",
            self.sanitize(&plant.name),
            self.sanitize(&plant.species),
            plant.planted.format(&self.date_format),
            plant.died.format(&self.date_format),
            self.sanitize(&plant.reason));
        self.connection.execute(graveyard_query)?;

        let info_query = format!(
            "DELETE FROM plants WHERE name='{}';",
            self.sanitize(&plant.name)
        );
        self.connection.execute(info_query)?;

        let logs_query = format!(
            "DELETE FROM activities WHERE plant='{}';",
            self.sanitize(&plant.name)
        );
        self.connection.execute(logs_query)?;

        let growth_query = format!(
            "DELETE FROM growth WHERE plant='{}';",
            self.sanitize(&plant.name)
        );
        self.connection.execute(growth_query)?;

        // move images to dead dir
        let dead_dir = self.plants_dir.join("dead");
        if !dead_dir.exists() {
            std::fs::create_dir_all(dead_dir.clone())?;
        }
        let dead_path = dead_dir.join(plant.name.clone());
        let plant_path = PathBuf::from(&self.plants_dir).join(plant.name);
        let _ = std::fs::rename(plant_path, dead_path);

        Ok(())
    }

    // Location Methods
    fn get_locations(&mut self) -> Result<Vec<Location>, Box<dyn StdErr>> {
        let query = "SELECT * FROM locations";
        let location_maps = self.read_rows(query, vec!["name", "outside"])?;
        let mut locations = vec![];
        for mut map in location_maps.into_iter() {
            map.insert("date_format".to_owned(), self.date_format.clone());
            let loc: Location = map.try_into()?;
            locations.push(loc);
        }

        Ok(locations)
    }

    fn get_location(&mut self, location_name: &str) -> Result<Location, Box<dyn StdErr>> {
        let query = format!("SELECT * FROM locations WHERE name LIKE '%{location_name}%'");
        let location_maps = self.read_rows(&query, vec!["name", "outside"])?;
        let map_fst = location_maps.first().ok_or(Error::LocationNotFound {
            name: location_name.to_owned(),
        })?;

        let lookup_fun = |key: &str| {
            map_fst.get(key).cloned().ok_or(Error::MissingValue {
                key: key.to_owned(),
            })
        };
        let location = Location {
            name: lookup_fun("name")?,
            outside: lookup_fun("outside")? == "1",
        };
        Ok(location)
    }

    fn write_location(&mut self, location: Location) -> Result<(), Box<dyn StdErr>> {
        let query = format!(
            "INSERT INTO locations (name,outside) VALUES ('{}','{}')",
            self.sanitize(&location.name),
            location.outside
        );
        self.connection.execute(query)?;
        Ok(())
    }

    // Log Methods
    fn get_logs(&mut self) -> Result<Vec<LogItem>, Box<dyn StdErr>> {
        let query = "SELECT * FROM activities";
        let logs_maps = self.read_rows(query, vec!["name", "date", "plant", "note"])?;
        let mut logs = vec![];
        for mut map in logs_maps.into_iter() {
            map.insert("date_format".to_owned(), self.date_format.clone());
            let log: LogItem = map.try_into()?;
            logs.push(log);
        }
        Ok(logs)
    }

    fn write_logs(&mut self, logs: Vec<LogItem>) -> Result<(), Box<dyn StdErr>> {
        let mut insert_strs = vec![];
        for log in logs.into_iter() {
            let note_str = if let Some(note) = log.note {
                format!("'{note}'")
            } else {
                "null".to_owned()
            };

            insert_strs.push(format!(
                "('{}','{}','{}',{})",
                self.sanitize(&log.activity),
                self.sanitize(&log.date.format(&self.date_format)),
                self.sanitize(&log.plant),
                self.sanitize(&note_str),
            ));
        }
        let query = format!(
            "INSERT INTO activities (name,date,plant,note) VALUES {};",
            insert_strs.join(", ")
        );
        self.connection.execute(query)?;
        Ok(())
    }

    // Growth Methods
    fn get_growth(&mut self) -> Result<Vec<GrowthItem>, Box<dyn StdErr>> {
        let query = "SELECT * FROM growth";
        let growth_maps = self.read_rows(
            query,
            vec!["plant", "date", "height_cm", "width_cm", "note", "health"],
        )?;
        let mut growth = vec![];
        for mut map in growth_maps.into_iter() {
            map.insert("date_format".to_owned(), self.date_format.clone());
            let item: GrowthItem = map.try_into()?;
            growth.push(item);
        }
        Ok(growth)
    }

    fn write_growths(&mut self, growth: Vec<GrowthItem>) -> Result<(), Box<dyn StdErr>> {
        if growth.is_empty() {
            return Ok(());
        }

        let mut insert_strs = vec![];
        for item in growth.into_iter() {
            let note_str = if let Some(note) = item.note {
                format!("'{}'", self.sanitize(&note))
            } else {
                "null".to_owned()
            };

            insert_strs.push(format!(
                "('{}','{}',{},{},{},{})",
                self.sanitize(&item.plant),
                self.sanitize(&item.date.format(&self.date_format)),
                self.sanitize(&item.height_cm),
                self.sanitize(&item.width_cm),
                &note_str,
                self.sanitize(&item.health)
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
        let plant_maps = self.read_rows(&query, vec!["num"])?;
        let plant_map = plant_maps.first().ok_or(Error::MissingValue {
            key: "num".to_owned(),
        })?;
        let cnt = plant_map.get("num").ok_or(Error::MissingValue {
            key: "num".to_owned(),
        })?;
        let exists = cnt.parse::<usize>()? > 0;
        Ok(exists)
    }

    fn species_exists(&mut self, species_name: &str) -> Result<bool, Box<dyn StdErr>> {
        let query =
            format!("SELECT COUNT(*) AS num FROM species WHERE name LIKE '%{species_name}%'");
        let species_maps = self.read_rows(&query, vec!["num"])?;
        let species_map = species_maps.first().ok_or(Error::MissingValue {
            key: "num".to_owned(),
        })?;
        let num = species_map.get("num").ok_or(Error::MissingValue {
            key: "num".to_owned(),
        })?;
        let exists = num.parse::<usize>()? > 0;
        Ok(exists)
    }
}
