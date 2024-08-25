use super::errors::Error;
use chrono::NaiveDate;
use database::file_backend::{load_json::load_dir, write_json::write_plants};
use plants::plant::{PlantInfo, PlantSpecies};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

fn load_old_plants(plants_dir: &PathBuf) -> Result<Vec<PlantJSON>, Error> {
    let plants: Vec<PlantJSON> = load_dir(plants_dir)?;
    Ok(plants)
}

fn convert_old_plants(plants: Vec<PlantJSON>, date_format: &str) -> Result<Vec<PlantInfo>, Error> {
    let mut new_plants = vec![];
    for old_plant in plants.into_iter() {
        let obtained = NaiveDate::parse_from_str(&old_plant.obtained, date_format)?;
        let auto_water = old_plant.auto_watering.try_into()?;
        let new_plant = PlantInfo {
            name: old_plant.plant_name,
            species: PlantSpecies::Other(old_plant.species_name),
            location: old_plant.current_location,
            origin: old_plant.origin,
            obtained,
            auto_water,
            notes: old_plant.plant_notes,
        };
        new_plants.push(new_plant);
    }
    Ok(new_plants)
}

fn save_new_plants(plants: Vec<PlantInfo>, plants_dir: &PathBuf) -> Result<(), Error> {
    write_plants(plants, plants_dir)?;
    Ok(())
}

pub fn port_plants(
    plants_dir_old: &PathBuf,
    date_format: &str,
    plants_dir_new: &PathBuf,
) -> Result<(), Error> {
    let old_plants = load_old_plants(plants_dir_old)?;
    let new_plants = convert_old_plants(old_plants, date_format)?;
    save_new_plants(new_plants, plants_dir_new)
}
