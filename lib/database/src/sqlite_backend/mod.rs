use super::{database_manager::DatabaseManager, file_backend::json_to_plant::load_images};
use chrono::NaiveDate;
use plants::{
    graveyard::GraveyardPlant,
    growth_item::GrowthItem,
    location::Location,
    log_item::LogItem,
    plant::{Plant, PlantInfo, PlantLocation, PlantSpecies},
    species::{Species, SunlightRequirement},
};
use sqlite::Connection;
use std::{collections::HashMap, error::Error as StdErr, path::PathBuf};

pub mod errors;
use errors::Error;

pub struct SQLiteDB {
    pub db_path: PathBuf,
    pub connection: Connection,
    pub date_format: String,
    pub plants_dir: PathBuf,
}

impl SQLiteDB {
    pub fn new(path: PathBuf) -> Result<SQLiteDB, Error> {
        let con = sqlite::open(path.clone())?;
        Ok(SQLiteDB {
            db_path: path,
            connection: con,
            date_format: "%d.%m.%Y".to_owned(),
            plants_dir: PathBuf::from("data").join("Plants"),
        })
    }

    pub fn read_rows(
        &mut self,
        query: &str,
        column_keys: Vec<&str>,
    ) -> Result<Vec<HashMap<String, String>>, Error> {
        let mut maps = vec![];
        let callback = |cols: &[(&str, Option<&str>)]| {
            let mut map = HashMap::new();
            for (key, val) in cols.into_iter() {
                let value = if let Some(val) = val { val } else { continue };
                if column_keys.contains(key) {
                    map.insert(format!("{}", key), format!("{}", value));
                }
            }
            maps.push(map);
            true
        };
        self.connection.iterate(query, callback)?;
        Ok(maps)
    }

    pub fn get_growth_plant(&mut self, plant_name: &str) -> Vec<GrowthItem> {
        let growth_query = format!("SELECT * FROM growth WHERE plant='{}'", plant_name);
        let mut growths = vec![];
        let growth_callback = |rows: &[(&str, Option<&str>)]| {
            for (key, value) in rows.iter() {
                let mut plant = "";
                let mut date = None;
                let mut height = -1.0;
                let mut width = -1.0;
                let mut note = None;

                let mut health = -1;
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

                if plant != "" && date.is_some() && height != -1.0 && width != -1.0 && health != -1
                {
                    growths.push(GrowthItem {
                        plant: plant.to_owned(),
                        date: date.unwrap(),
                        height_cm: height,
                        width_cm: width,
                        note,
                        health,
                    });
                }
            }
            true
        };
        self.connection
            .iterate(growth_query, growth_callback)
            .unwrap();
        growths
    }

    pub fn get_logs_plant(&mut self, plant_name: &str) -> Vec<LogItem> {
        let mut logs = vec![];
        let log_query = format!("SELECT * FROM activities WHERE plant={}", plant_name);
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
        self.connection.iterate(log_query, log_callback).unwrap();
        logs
    }
}

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
        for val_map in info_maps.into_iter() {
            let read_fun = |key: &str| {
                val_map.get(key).cloned().ok_or(Error::MissingValue {
                    key: key.to_owned(),
                })
            };
            let species_name = read_fun("species")?;
            let species = self
                .get_species(&species_name)
                .map(|sp| PlantSpecies::Species(Box::new(sp)))
                .unwrap_or(PlantSpecies::Other(species_name));
            let location_name = read_fun("location")?;
            let location = self
                .get_location(&location_name)
                .map(|loc| PlantLocation::Location(Box::new(loc)))
                .unwrap_or(PlantLocation::Other(location_name));
            let obtained_str = read_fun("obtained")?;
            let auto_water = read_fun("auto_water")? == "1";
            let notes = read_fun("notes")?;
            infos.push(PlantInfo {
                name: read_fun("name")?,
                species,
                location,
                origin: read_fun("origin")?,
                obtained: NaiveDate::parse_from_str(&obtained_str, &self.date_format)?,
                auto_water,
                notes: notes.split(", ").map(|s| s.to_owned()).collect(),
            });
        }

        let mut plants = vec![];
        for plant in infos.into_iter() {
            let growths = self.get_growth_plant(&plant.name);
            let logs = self.get_logs_plant(&plant.name);

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
        let plant_map = info_maps.first().ok_or(Error::PlantNotFound {
            name: plant_name.to_owned(),
        })?;
        let lookup_fun = |key: &str| {
            plant_map.get(key).cloned().ok_or(Error::MissingValue {
                key: key.to_owned(),
            })
        };
        let species_name = lookup_fun("species")?;
        let species = self
            .get_species(&species_name)
            .map(|sp| PlantSpecies::Species(Box::new(sp)))
            .unwrap_or(PlantSpecies::Other(species_name));
        let location_name = lookup_fun("location")?;
        let location = self
            .get_location(&location_name)
            .map(|loc| PlantLocation::Location(Box::new(loc)))
            .unwrap_or(PlantLocation::Other(location_name));
        let obtained_str = lookup_fun("obtained")?;
        let auto_water = lookup_fun("auto_water")? == "1";
        let notes = lookup_fun("notes")?
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        let info = PlantInfo {
            name: lookup_fun("name")?,
            species,
            location,
            origin: lookup_fun("origin")?,
            obtained: NaiveDate::parse_from_str(&obtained_str, &self.date_format)?,
            auto_water,
            notes,
        };

        let growths = self.get_growth_plant(&plant_name);
        let logs = self.get_logs_plant(&plant_name);

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
        for plant in plants {
            let notes_str = if plant.notes.is_empty() {
                "null".to_owned()
            } else {
                format!("'{}'", plant.notes.join(", "))
            };

            let plant_str = format!(
                ", ('{}','{}','{}','{}','{}','{}')",
                plant.name,
                plant.species,
                plant.location,
                plant.obtained,
                plant.auto_water,
                notes_str
            );
            plant_query += &plant_str;
        }
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
        for species_map in species_maps {
            let lookup_fun = |key: &str| {
                species_map.get(key).cloned().ok_or(Error::MissingValue {
                    key: key.to_owned(),
                })
            };
            let sun_str = lookup_fun("sunlight")?;
            let sunlight = sun_str
                .parse::<SunlightRequirement>()
                .map_err(|_| Error::BadValue {
                    key: "sunlight".to_owned(),
                    value: sun_str,
                })?;

            let temp_min_str = lookup_fun("temp_min")?;
            let temp_min = temp_min_str.parse::<f32>()?;
            let temp_max_str = lookup_fun("temp_max")?;
            let temp_max = temp_max_str.parse::<f32>()?;
            let temp_min_opt_str = lookup_fun("opt_temp_min")?;
            let opt_temp_min = temp_min_opt_str.parse::<f32>()?;
            let temp_max_opt_str = lookup_fun("opt_temp_max")?;
            let opt_temp_max = temp_max_opt_str.parse::<f32>()?;
            let ph_min_str = lookup_fun("ph_min")?;
            let ph_min = ph_min_str.parse::<f32>()?;
            let ph_max_str = lookup_fun("ph_max")?;
            let ph_max = ph_max_str.parse::<f32>()?;

            let dist = species_map
                .get("planting_distance")
                .map(|d| d.parse::<f32>())
                .transpose()?;
            let watering_notes = species_map
                .get("watering_notes")
                .map(|s| s.split(", ").map(|x| x.to_owned()).collect())
                .unwrap_or(vec![]);

            let avg_watering_days = species_map
                .get("avg_watering_days")
                .map(|d| d.parse::<i32>())
                .transpose()?;
            let avg_fertilizing_days = species_map
                .get("avg_watering_days")
                .map(|d| d.parse::<i32>())
                .transpose()?;

            let fertilizing_notes = species_map
                .get("fertilizing_notes")
                .map(|s| s.split(", ").map(|x| x.to_owned()).collect())
                .unwrap_or(vec![]);
            let pruning_notes = species_map
                .get("pruning_notes")
                .map(|s| s.split(", ").map(|x| x.to_owned()).collect())
                .unwrap_or(vec![]);
            let companions = species_map
                .get("companions")
                .map(|s| s.split(", ").map(|x| x.to_owned()).collect())
                .unwrap_or(vec![]);
            let additional_notes = species_map
                .get("additional_notes")
                .map(|s| s.split(", ").map(|x| x.to_owned()).collect())
                .unwrap_or(vec![]);

            species.push(Species {
                name: lookup_fun("name")?,
                scientific_name: lookup_fun("scientific_name")?,
                genus: lookup_fun("genus")?,
                family: lookup_fun("family")?,
                sunlight,
                temp_min,
                temp_max,
                opt_temp_min,
                opt_temp_max,
                planting_distance: dist,
                ph_min,
                ph_max,
                watering_notes,
                fertilizing_notes,
                avg_watering_days,
                avg_fertilizing_days,
                pruning_notes,
                companions,
                additional_notes,
            });
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
        let map_fst = species_map.first().ok_or(Error::SpeciesNotFound {
            name: species_name.to_owned(),
        })?;
        let lookup_fun = |key: &str| {
            map_fst.get(key).cloned().ok_or(Error::MissingValue {
                key: key.to_owned(),
            })
        };
        let sunlight = lookup_fun("sunlight")?.parse::<SunlightRequirement>()?;

        let temp_min = lookup_fun("temp_min")?.parse::<f32>()?;
        let temp_max = lookup_fun("temp_max")?.parse::<f32>()?;
        let opt_temp_min = lookup_fun("temp_min_opt")?.parse::<f32>()?;
        let opt_temp_max = lookup_fun("temp_max_opt")?.parse::<f32>()?;
        let ph_min = lookup_fun("ph_min")?.parse::<f32>()?;
        let ph_max = lookup_fun("ph_max")?.parse::<f32>()?;

        let planting_distance = map_fst
            .get("planting_distance")
            .map(|s| s.parse::<f32>())
            .transpose()?;
        let avg_watering_days = map_fst
            .get("avg_watering_days")
            .map(|s| s.parse::<i32>())
            .transpose()?;
        let avg_fertilizing_days = map_fst
            .get("avg_fertilizing_days")
            .map(|s| s.parse::<i32>())
            .transpose()?;

        let watering_notes = map_fst
            .get("watering_notes")
            .map(|s| s.split(", ").map(|s| s.to_owned()).collect())
            .unwrap_or(vec![]);
        let fertilizing_notes = map_fst
            .get("fertilizing_notes")
            .map(|s| s.split(", ").map(|s| s.to_owned()).collect())
            .unwrap_or(vec![]);
        let pruning_notes = map_fst
            .get("pruning_notes")
            .map(|s| s.split(", ").map(|s| s.to_owned()).collect())
            .unwrap_or(vec![]);
        let companions = map_fst
            .get("companions")
            .map(|s| s.split(", ").map(|s| s.to_owned()).collect())
            .unwrap_or(vec![]);
        let additional_notes = map_fst
            .get("additional_notes")
            .map(|s| s.split(", ").map(|s| s.to_owned()).collect())
            .unwrap_or(vec![]);

        let species = Species {
            name: lookup_fun("name")?,
            scientific_name: lookup_fun("scientific_name")?,
            family: lookup_fun("family")?,
            genus: lookup_fun("genus")?,
            sunlight,
            temp_min,
            temp_max,
            opt_temp_max,
            opt_temp_min,
            ph_min,
            ph_max,
            planting_distance,
            avg_watering_days,
            avg_fertilizing_days,
            watering_notes,
            fertilizing_notes,
            pruning_notes,
            companions,
            additional_notes,
        };

        Ok(species)
    }

    fn write_species(&mut self, species: Species) -> Result<(), Box<dyn StdErr>> {
        let plant_distance_str = species
            .planting_distance
            .map(|f| format!("'{}'", f))
            .unwrap_or("null".to_owned());
        let avg_watering_days_str = species
            .avg_watering_days
            .map(|f| format!("'{}'", f))
            .unwrap_or("null".to_owned());
        let avg_fertilizing_str = species
            .avg_fertilizing_days
            .map(|f| format!("'{}'", f))
            .unwrap_or("null".to_owned());

        let watering_notes_str = if species.watering_notes.is_empty() {
            "null".to_owned()
        } else {
            format!("'{}'", species.watering_notes.join(", "))
        };
        let fertilizing_notes_str = if species.fertilizing_notes.is_empty() {
            "null".to_owned()
        } else {
            format!("'{}'", species.fertilizing_notes.join(", "))
        };
        let pruning_str = if species.pruning_notes.is_empty() {
            "null".to_owned()
        } else {
            format!("'{}'", species.pruning_notes.join(", "))
        };
        let notes_str = if species.additional_notes.is_empty() {
            "null".to_owned()
        } else {
            format!("'{}'", species.additional_notes.join(", "))
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
            ('{}','{}','{}','{}','{}',{},{},{},{},{},{},{},{},{},{},{},{},{})",
            species.name,
            species.scientific_name,
            species.genus,
            species.family,
            species.sunlight,
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
        for map in graveyard_maps {
            let lookup_fun = |key: &str| {
                map.get(key).cloned().ok_or(Error::MissingValue {
                    key: key.to_owned(),
                })
            };
            let planted = NaiveDate::parse_from_str(&lookup_fun("planted")?, &self.date_format)?;
            let died = NaiveDate::parse_from_str(&lookup_fun("died")?, &self.date_format)?;
            graveyard.push(GraveyardPlant {
                name: lookup_fun("name")?,
                species: lookup_fun("species")?,
                planted,
                died,
                reason: lookup_fun("reason")?,
            });
        }

        Ok(graveyard)
    }

    fn kill_plant(&mut self, plant: GraveyardPlant) -> Result<(), Box<dyn StdErr>> {
        let graveyard_query = format!(
            "INSERT INTO graveyard (name,species,planted,died,reason) values ('{}','{}','{}','{}','{}')",plant.name,plant.species,plant.planted.format(&self.date_format),plant.died.format(&self.date_format),plant.reason
        );
        self.connection.execute(graveyard_query)?;

        let info_query = format!("DELETE FROM plants WHERE name='{}';", plant.name);
        self.connection.execute(info_query)?;

        let logs_query = format!("DELETE FROM activities WHERE plant='{}';", plant.name);
        self.connection.execute(logs_query)?;

        let growth_query = format!("DELETE FROM growth WHERE plant='{}';", plant.name);
        self.connection.execute(growth_query)?;

        // move images to dead dir
        let dead_dir = self.plants_dir.join("dead");
        if !dead_dir.exists() {
            std::fs::create_dir_all(dead_dir.clone())?;
        }
        let dead_path = dead_dir.join(plant.name.clone());
        let plant_path = PathBuf::from(&self.plants_dir).join(plant.name);
        std::fs::rename(plant_path, dead_path).map_err(|err| Box::new(err))?;

        Ok(())
    }

    // Location Methods
    fn get_locations(&mut self) -> Result<Vec<Location>, Box<dyn StdErr>> {
        let query = "SELECT * FROM locations";
        let location_maps = self.read_rows(query, vec!["name"])?;
        let mut locations = vec![];
        for map in location_maps {
            let lookup_fun = |key: &str| {
                map.get(key).cloned().ok_or(Error::MissingValue {
                    key: key.to_owned(),
                })
            };
            locations.push(Location {
                name: lookup_fun("name")?,
                outside: lookup_fun("outside")? == "1",
            });
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

    // Log Methods
    fn get_logs(&mut self) -> Result<Vec<LogItem>, Box<dyn StdErr>> {
        let query = "SELECT * FROM activities";
        let logs_maps = self.read_rows(query, vec!["name", "date", "plant", "note"])?;
        let mut logs = vec![];
        for map in logs_maps {
            let lookup_fun = |key: &str| {
                map.get(key).cloned().ok_or(Error::MissingValue {
                    key: key.to_owned(),
                })
            };
            logs.push(LogItem {
                activity: lookup_fun("name")?,
                date: NaiveDate::parse_from_str(&lookup_fun("date")?, &self.date_format)?,
                plant: lookup_fun("plant")?,
                note: map.get("note").cloned(),
            });
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
                log.activity,
                log.date.format(&self.date_format),
                log.plant,
                note_str,
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
        for map in growth_maps {
            let lookup_fun = |key: &str| {
                map.get(key).cloned().ok_or(Error::MissingValue {
                    key: key.to_owned(),
                })
            };
            let height_cm = lookup_fun("height_cm")?.parse::<f32>()?;
            let width_cm = lookup_fun("width_cm")?.parse::<f32>()?;
            let health = lookup_fun("health")?.parse::<i32>()?;
            growth.push(GrowthItem {
                plant: lookup_fun("plant")?,
                date: NaiveDate::parse_from_str(&lookup_fun("date")?, &self.date_format)?,
                height_cm,
                width_cm,
                health,
                note: map.get("note").cloned(),
            });
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