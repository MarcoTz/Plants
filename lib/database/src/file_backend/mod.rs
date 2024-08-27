pub mod errors;
pub mod json_to_plant;
pub mod load_csv;
pub mod load_json;
pub mod write_csv;
pub mod write_json;

use errors::Error;
use json_to_plant::load_plants;
use load_csv::{load_graveyard, load_locations};
use load_json::load_species;
use write_csv::{write_activities, write_graveyard, write_growth};
use write_json::{write_plants, write_species};

use crate::database_manager::DatabaseManager;
use plants::{
    graveyard::GraveyardPlant,
    growth_item::GrowthItem,
    location::Location,
    log_item::LogItem,
    named::Named,
    plant::{Plant, PlantInfo, PlantSpecies},
    species::Species,
};
use std::{fs::remove_file, io::Error as IOError, path::PathBuf};

pub struct FileDB {
    pub plants_dir: PathBuf,
    pub species_dir: PathBuf,
    pub location_file: PathBuf,
    logs_dir: PathBuf,
    graveyard_csv: String,
    growth_csv: String,
    activities_csv: String,
    pub date_format: String,

    pub plants_cache: Vec<Plant>,
    pub graveyard_cache: Vec<GraveyardPlant>,
    pub species_cache: Vec<Species>,
    pub location_cache: Vec<Location>,
}

impl Default for FileDB {
    fn default() -> Self {
        let data_dir: PathBuf = "data".into();
        FileDB {
            plants_dir: data_dir.join("Plants"),
            species_dir: data_dir.join("Species"),
            location_file: data_dir.join("Locations.csv"),
            logs_dir: data_dir.join("Logs"),
            graveyard_csv: "Graveyard.csv".to_owned(),
            growth_csv: "Growth.csv".to_owned(),
            activities_csv: "Activities.csv".to_owned(),
            date_format: "%d.%m.%Y".to_owned(),
            plants_cache: vec![],
            graveyard_cache: vec![],
            species_cache: vec![],
            location_cache: vec![],
        }
    }
}

impl FileDB {
    pub fn get_activities_filepath(&self) -> PathBuf {
        self.logs_dir.join(self.activities_csv.clone())
    }

    pub fn get_graveyard_filepath(&self) -> PathBuf {
        self.logs_dir.join(self.graveyard_csv.clone())
    }

    pub fn get_growth_filepath(&self) -> PathBuf {
        self.logs_dir.join(self.growth_csv.clone())
    }

    fn load_plants(&mut self) -> Result<(), Error> {
        log::info!("Loading plants from json and csv");
        let activity_file = self.get_activities_filepath();
        let growth_file = self.get_growth_filepath();
        let plants = load_plants(
            &self.plants_dir,
            &self.species_dir,
            &activity_file,
            &growth_file,
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
        let graveyard_file = self.get_graveyard_filepath();
        let graveyard = load_graveyard(&graveyard_file)?;
        self.graveyard_cache = graveyard;
        Ok(())
    }

    fn load_locations(&mut self) -> Result<(), Error> {
        log::info!("Loading Locations from csv");
        let locations = load_locations(&self.location_file)?;
        self.location_cache = locations;
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

    fn get_locations(&mut self) -> Result<Vec<Location>, crate::errors::Error> {
        if self.location_cache.is_empty() {
            self.load_locations()?;
        }
        Ok(self.location_cache.clone())
    }

    fn get_location(&mut self, location_name: &str) -> Result<Location, crate::errors::Error> {
        if self.location_cache.is_empty() {
            self.load_locations()?;
        }
        let err: crate::errors::Error = Error::LocationNotFound(location_name.to_owned()).into();
        self.location_cache
            .iter()
            .filter(|loc| loc.get_name() == location_name)
            .cloned()
            .collect::<Vec<Location>>()
            .first()
            .cloned()
            .ok_or(err)
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
        write_activities(logs, &self.get_activities_filepath())?;
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
            .filter(|pl| pl.info.location.get_name() == location)
            .cloned()
            .collect())
    }

    fn write_growths(&mut self, growth: Vec<GrowthItem>) -> Result<(), crate::errors::Error> {
        write_growth(growth, &self.get_growth_filepath())?;
        Ok(())
    }

    fn write_plant(&mut self, plant: PlantInfo) -> Result<(), crate::errors::Error> {
        write_plants(vec![plant], &self.plants_dir)?;
        Ok(())
    }

    fn write_species(&mut self, species: Species) -> Result<(), crate::errors::Error> {
        write_species(vec![species], &self.species_dir)?;
        Ok(())
    }

    fn kill_plant(&mut self, plant: GraveyardPlant) -> Result<(), crate::errors::Error> {
        let name = plant.name.clone();
        write_graveyard(vec![plant], &self.get_graveyard_filepath())?;
        let plant_filename = name.replace(' ', "") + ".json";
        let plant_path = PathBuf::from(&self.plants_dir).join(plant_filename.clone());
        remove_file(plant_path.clone()).map_err(|err| <IOError as Into<Error>>::into(err))?;
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
