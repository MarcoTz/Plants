use super::Port;
use crate::errors::Error;
use chrono::NaiveDate;
use database::file_backend::{load_json::load_json, write_json::write_plants};
use plants::plant::{PlantInfo, PlantLocation, PlantSpecies};
use serde::{Deserialize, Serialize};
use std::{fs::read_dir, path::PathBuf};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlantJSON {
    pub auto_watering: BoolOrString,
    pub current_location: String,
    pub obtained: String,
    pub origin: String,
    pub plant_health: String,
    pub plant_name: String,
    pub plant_notes: Vec<String>,
    pub species_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    Str(String),
}

impl From<bool> for BoolOrString {
    fn from(b: bool) -> BoolOrString {
        BoolOrString::Bool(b)
    }
}
impl TryInto<bool> for BoolOrString {
    type Error = Error;
    fn try_into(self) -> Result<bool, Self::Error> {
        let new_b = match self {
            BoolOrString::Bool(b) => Ok(b),
            BoolOrString::Str(st) => {
                if st.as_str() == "y" {
                    Ok(true)
                } else if st.as_str() == "n" {
                    Ok(false)
                } else {
                    st.to_lowercase().trim().parse::<bool>()
                }
            }
        }?;
        Ok(new_b)
    }
}

impl Port<Vec<PlantInfo>> for Vec<PlantJSON> {
    type LoadArgs = PathBuf;
    type SaveArgs = PathBuf;
    type ConvertArgs = String;

    fn load_old(plants_dir: &Self::LoadArgs) -> Result<Vec<PlantJSON>, Error> {
        log::info!("Loading old plant infos");
        let mut plants = vec![];
        let contents = read_dir(plants_dir)?;
        for plant_file in contents {
            let file = plant_file?;
            let plant: PlantJSON = load_json(&file.path())?;
            plants.push(plant);
        }
        Ok(plants)
    }

    fn convert(self, date_format: &Self::ConvertArgs) -> Result<Vec<PlantInfo>, Error> {
        log::info!("Converting plant infos");
        let mut new_plants = vec![];
        for old_plant in self.into_iter() {
            let obtained = NaiveDate::parse_from_str(&old_plant.obtained, date_format)?;
            let auto_water = old_plant.auto_watering.try_into()?;
            let new_plant = PlantInfo {
                name: old_plant.plant_name,
                species: PlantSpecies::Other(old_plant.species_name),
                location: PlantLocation::Other(old_plant.current_location),
                origin: old_plant.origin,
                obtained,
                auto_water,
                notes: old_plant.plant_notes,
            };
            new_plants.push(new_plant);
        }
        Ok(new_plants)
    }

    fn save_new(plants: Vec<PlantInfo>, plants_dir: &Self::SaveArgs) -> Result<(), Error> {
        log::info!("Saving new Plants");
        write_plants(plants, plants_dir)?;
        Ok(())
    }
}
