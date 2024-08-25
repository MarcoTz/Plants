pub mod csv_to_growth_item;
pub mod csv_to_log_item;
pub mod errors;
pub mod json_to_plant;
pub mod load_csv;
pub mod load_json;
pub mod write_csv;
pub mod write_json;

use errors::{AccessType, Error, FSError};
use json_to_plant::load_plants;
use load_csv::load_graveyard;
use load_json::load_species;
use write_csv::{write_activities, write_graveyard, write_growth};
use write_json::{write_plants, write_species};

use crate::database_manager::{DatabaseManager, PlantJSON};
use plants::{
    graveyard::GraveyardPlant,
    growth_item::GrowthItem,
    log_item::LogItem,
    plant::{Plant, PlantSpecies},
    species::Species,
};
use std::{fs::remove_file, path};

pub struct FileDB {
    pub plants_dir: String,
    pub species_dir: String,
    logs_dir: String,
    graveyard_csv: String,
    growth_csv: String,
    activities_csv: String,
    pub date_format: String,

    pub plants_cache: Vec<Plant>,
    pub graveyard_cache: Vec<GraveyardPlant>,
    pub species_cache: Vec<Species>,
}

fn get_path_from_buf(logs_dir: &str, file_name: &str) -> Result<String, Error> {
    let file_path = path::Path::new(logs_dir).join(file_name);
    match file_path.to_str() {
        None => Err(FSError {
            file_name: file_name.to_owned(),
            err_msg: "Could not find path".to_owned(),
            access: AccessType::Read,
        }
        .into()),
        Some(st) => Ok(st.to_owned()),
    }
}

impl Default for FileDB {
    fn default() -> Self {
        FileDB {
            plants_dir: "data/Plants".to_owned(),
            species_dir: "data/PlantSpecies".to_owned(),
            logs_dir: "data/Logs".to_owned(),
            graveyard_csv: "Graveyard.csv".to_owned(),
            growth_csv: "Growth.csv".to_owned(),
            activities_csv: "Activities.csv".to_owned(),
            date_format: "%d.%m.%Y".to_owned(),
            plants_cache: vec![],
            graveyard_cache: vec![],
            species_cache: vec![],
        }
    }
}

impl FileDB {
    pub fn get_activities_filepath(&self) -> Result<String, Error> {
        get_path_from_buf(&self.logs_dir, &self.activities_csv)
    }

    pub fn get_graveyard_filepath(&self) -> Result<String, Error> {
        get_path_from_buf(&self.logs_dir, &self.graveyard_csv)
    }

    pub fn get_growth_filepath(&self) -> Result<String, Error> {
        get_path_from_buf(&self.logs_dir, &self.growth_csv)
    }

    fn load_plants(&mut self) -> Result<(), Error> {
        log::info!("Loading plants from json and csv");
        let activity_file = self.get_activities_filepath()?;
        let growth_file = self.get_growth_filepath()?;
        let plants = load_plants(
            &self.plants_dir,
            &self.species_dir,
            &activity_file,
            &growth_file,
            &self.date_format,
        )?;
        self.plants_cache = plants;
        Ok(())
    }

    fn load_species(&mut self) -> Result<(), Error> {
        log::info!("Loading species from json");
        let species = load_species(&self.species_dir)?;
        self.species_cache = species;
        Ok(())
    }

    fn load_graveyard(&mut self) -> Result<(), Error> {
        log::info!("Loading graveyard from csv");
        let graveyard_file = self.get_graveyard_filepath()?;
        let graveyard = load_graveyard(&graveyard_file)?;
        self.graveyard_cache = graveyard;
        Ok(())
    }
}

impl DatabaseManager for FileDB {
    fn get_all_plants(&mut self) -> Result<Vec<Plant>, crate::errors::Error> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }
        Ok(self.plants_cache.clone())
    }

    fn get_num_plants(&mut self) -> Result<i32, crate::errors::Error> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }
        Ok(self.plants_cache.len() as i32)
    }

    fn get_plant(&mut self, plant_name: &str) -> Result<Plant, crate::errors::Error> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }
        self.plants_cache
            .iter()
            .filter(|pl| pl.info.name == plant_name)
            .cloned()
            .collect::<Vec<Plant>>()
            .first()
            .cloned()
            .ok_or(Error::PlantNotFound(plant_name.to_owned()).into())
    }

    fn get_all_species(&mut self) -> Result<Vec<Species>, crate::errors::Error> {
        if self.species_cache.is_empty() {
            self.load_species()?;
        }
        Ok(self.species_cache.clone())
    }

    fn get_species(&mut self, species_name: &str) -> Result<Species, crate::errors::Error> {
        if self.species_cache.is_empty() {
            self.load_species()?;
        }
        self.species_cache
            .iter()
            .filter(|sp| sp.name == species_name)
            .cloned()
            .collect::<Vec<Species>>()
            .first()
            .cloned()
            .ok_or(Error::SpeciesNotFound(species_name.to_owned()).into())
    }

    fn get_graveyard(&mut self) -> Result<Vec<GraveyardPlant>, crate::errors::Error> {
        if self.graveyard_cache.is_empty() {
            self.load_graveyard()?;
        }
        Ok(self.graveyard_cache.clone())
    }

    fn get_plants_species(
        &mut self,
        species_name: &str,
    ) -> Result<Vec<Plant>, crate::errors::Error> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }
        let species_plants = self
            .plants_cache
            .iter()
            .filter(|plant| match &plant.info.species {
                PlantSpecies::Other(name) => name == species_name,
                PlantSpecies::Species(sp) => sp.name == species_name,
            })
            .cloned()
            .collect();
        Ok(species_plants)
    }

    fn plant_exists(&mut self, plant_name: &str) -> Result<bool, crate::errors::Error> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }

        Ok(self
            .plants_cache
            .iter()
            .any(|pl| pl.info.name == plant_name))
    }

    fn species_exists(&mut self, species_name: &str) -> Result<bool, crate::errors::Error> {
        if self.species_cache.is_empty() {
            self.load_species()?;
        }
        Ok(self.species_cache.iter().any(|sp| sp.name == species_name))
    }

    fn write_logs(&mut self, logs: Vec<LogItem>) -> Result<(), crate::errors::Error> {
        write_activities(logs, &self.get_activities_filepath()?)?;
        Ok(())
    }

    fn get_plants_by_location(
        &mut self,
        location: String,
    ) -> Result<Vec<Plant>, crate::errors::Error> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }

        Ok(self
            .plants_cache
            .iter()
            .filter(|pl| pl.info.location == location)
            .cloned()
            .collect())
    }

    fn write_growths(&mut self, growth: Vec<GrowthItem>) -> Result<(), crate::errors::Error> {
        write_growth(growth, &self.get_growth_filepath()?)?;
        Ok(())
    }

    fn write_plant(&mut self, plant: PlantJSON) -> Result<(), crate::errors::Error> {
        write_plants(vec![plant], &self.plants_dir)?;
        Ok(())
    }

    fn write_species(&mut self, species: Species) -> Result<(), crate::errors::Error> {
        write_species(vec![species], &self.species_dir)?;
        Ok(())
    }

    fn kill_plant(&mut self, plant: GraveyardPlant) -> Result<(), crate::errors::Error> {
        let name = plant.name.clone();
        write_graveyard(vec![plant], &self.get_graveyard_filepath()?)?;
        let plant_filename = name.replace(' ', "") + ".json";
        let plant_path = path::Path::new(&self.plants_dir).join(plant_filename.clone());
        remove_file(plant_path).map_err(|_| {
            Error::FSError(FSError {
                file_name: plant_filename,
                err_msg: "Could not remove file".to_owned(),
                access: AccessType::Write,
            })
        })?;
        self.plants_cache = self
            .plants_cache
            .iter()
            .filter(|pl| pl.info.name == name)
            .cloned()
            .collect();
        //write plants

        Ok(())
    }
}
