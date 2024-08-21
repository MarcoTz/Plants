use super::{
    errors::{AccessType, Error, FSError},
    json_to_plant::load_plants,
    load_csv::load_graveyard,
    load_json::load_species,
};
use crate::database_manager::DatabaseManager;
use plants::{graveyard::GraveyardPlant, plant::Plant, species::Species};
use std::path;

pub struct FileDB {
    pub plants_dir: String,
    pub species_dir: String,
    logs_dir: String,
    graveyard_csv: String,
    growth_csv: String,
    activities_csv: String,
    pub date_format: String,
    //REMOVE later
    pub plants_out_dir: String,
    pub species_out_dir: String,
    pub graveyard_out: String,
    pub activities_out: String,
    pub growth_out: String,

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
impl FileDB {
    pub fn get_default() -> FileDB {
        FileDB {
            plants_dir: "data/Plants".to_owned(),
            species_dir: "data/PlantSpecies".to_owned(),
            logs_dir: "data/Logs".to_owned(),
            graveyard_csv: "Graveyard.csv".to_owned(),
            growth_csv: "Growth.csv".to_owned(),
            activities_csv: "Activities.csv".to_owned(),
            date_format: "%d.%m.%Y".to_owned(),
            plants_out_dir: "data_new/Plants".to_owned(),
            species_out_dir: "data_new/Species".to_owned(),
            graveyard_out: "data_new/Logs/Graveyard.csv".to_owned(),
            activities_out: "data_new/Logs/Activities.csv".to_owned(),
            growth_out: "data_new/Logs/Growth.csv".to_owned(),
            plants_cache: vec![],
            graveyard_cache: vec![],
            species_cache: vec![],
        }
    }
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

    fn get_all_species(&mut self) -> Result<Vec<Species>, crate::errors::Error> {
        if self.species_cache.is_empty() {
            self.load_species()?;
        }
        Ok(self.species_cache.clone())
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
            .filter(|plant| {
                plant.species.clone().is_some_and(|sp| {
                    sp.name.to_lowercase().trim() == species_name.to_lowercase().trim()
                })
            })
            .cloned()
            .collect();
        Ok(species_plants)
    }
}
