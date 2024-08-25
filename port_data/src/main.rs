mod errors;
mod port_plants;

use plants::species::Species;
use port_plants::port_plants;
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};

const DATA_DIR_OLD: &str = "data_old";
const DATA_DIR_NEW: &str = "data";
const DATE_FORMAT: &str = "%d.%m.%Y";

pub fn main() {
    let in_dir = PathBuf::from(DATA_DIR_OLD);
    let out_dir = PathBuf::from(DATA_DIR_NEW);
    match port_plants(&in_dir, DATE_FORMAT, &out_dir) {
        Ok(()) => println!("Successfully ported plants"),
        Err(err) => println!("{err:?}"),
    };

    //1. Load all plants and save them again, ensuring all fields have the correct types
    //  species needs to be either species or string
    //2. Do the same for all species
    //3. Add health to growth logs (currnent health for last log
    //4. All plant images need to be in a directory with the plants
    //  directory structure should be
    //      | - plants
    //          | - plant_name
    //              | - image1.jpg
    //              | - image2.jpg
    //              | - ...
    //              | - data.json
    //          | - plant_name
    //              | - ...
    //
    //
    /*let last_health = plant_json.plant_health.parse::<i32>()?;
    let mut last_growth =
        plant_growth
            .pop()
            .ok_or(Error::PlantError(plants::errors::Error::GrowthError(
                plant_info.name.clone(),
            )))?;
    last_growth.health = last_health;
    plant_growth.push(last_growth);*/
    println!()
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum FloatOrIntOrString {
    Int(i32),
    Float(f32),
    Str(String),
}

impl TryInto<i32> for FloatOrIntOrString {
    type Error = String;
    fn try_into(self) -> Result<i32, Self::Error> {
        let new_int = match self {
            FloatOrIntOrString::Int(i) => Ok(i),
            FloatOrIntOrString::Str(st) => st.parse::<i32>(),
            FloatOrIntOrString::Float(f) => Ok(f as i32),
        }
        .map_err(|_| "Cold not parse int")?;
        Ok(new_int)
    }
}
impl TryInto<f32> for FloatOrIntOrString {
    type Error = String;
    fn try_into(self) -> Result<f32, Self::Error> {
        let new_fl = match self {
            FloatOrIntOrString::Int(i) => Ok(i as f32),
            FloatOrIntOrString::Str(st) => st.parse::<f32>(),
            FloatOrIntOrString::Float(f) => Ok(f),
        }
        .map_err(|_| "Could not parse float")?;
        Ok(new_fl)
    }
}

fn option_try<U, T: TryInto<U>>(opt: Option<T>) -> Result<Option<U>, T::Error> {
    match opt {
        None => Ok(None),
        Some(m_u) => {
            let u = m_u.try_into()?;
            Ok(Some(u))
        }
    }
}
#[derive(Deserialize, Clone)]
pub struct SpeciesJSON {
    name: String,
    scientific_name: String,
    species_type: String,
    sunlight_requirements: String,
    temperature_min: FloatOrIntOrString,
    temperature_max: FloatOrIntOrString,
    optimal_temperature_min: FloatOrIntOrString,
    optimal_temperature_max: FloatOrIntOrString,
    plant_distance_cm: Option<FloatOrIntOrString>,
    ph_min: FloatOrIntOrString,
    ph_max: FloatOrIntOrString,
    avg_watering_days: Option<FloatOrIntOrString>,
    watering_notes: Vec<String>,
    avg_fertilizing_days: Option<FloatOrIntOrString>,
    fertilizing_notes: Vec<String>,
    pruning_notes: Vec<String>,
    companions: Vec<String>,
    additional_notes: Vec<String>,
}

impl TryInto<Species> for SpeciesJSON {
    type Error = String;

    fn try_into(self) -> Result<Species, Self::Error> {
        log::info!("Loading species {} from JSON", self.name);
        let new_temp_min = self.temperature_min.try_into()?;
        let new_temp_max = self.temperature_max.try_into()?;
        let new_opt_min = self.optimal_temperature_min.try_into()?;
        let new_opt_max = self.optimal_temperature_max.try_into()?;
        let new_ph_min = self.ph_min.try_into()?;
        let new_ph_max = self.ph_max.try_into()?;
        let new_dist = option_try(self.plant_distance_cm)?;
        let new_avg_water = option_try(self.avg_watering_days)?;
        let new_avg_fertilizing = option_try(self.avg_fertilizing_days)?;
        let new_sunlight = FromStr::from_str(&self.sunlight_requirements)
            .map_err(|_| "Could not parse sunlight")?;
        Ok(Species {
            name: self.name,
            scientific_name: self.scientific_name,
            genus: "".to_owned(),
            family: "".to_owned(),
            sunlight: new_sunlight,
            temp_min: new_temp_min,
            temp_max: new_temp_max,
            opt_temp_min: new_opt_min,
            opt_temp_max: new_opt_max,
            ph_min: new_ph_min,
            ph_max: new_ph_max,
            planting_distance: new_dist,
            watering_notes: self.watering_notes,
            avg_watering_days: new_avg_water,
            fertilizing_notes: self.fertilizing_notes,
            avg_fertilizing_days: new_avg_fertilizing,
            pruning_notes: self.pruning_notes,
            companions: self.companions,
            additional_notes: self.additional_notes,
        })
    }
}

/*impl From<(&Plant, String)> for PlantJSON {
    fn from((plant, date_format): (&Plant, String)) -> PlantJSON {
        PlantJSON {
            plant_name: plant.info.name.clone(),
            species_name: match &plant.info.species {
                PlantSpecies::Other(name) => name.clone(),
                PlantSpecies::Species(sp) => sp.name.clone(),
            },
            auto_watering: BoolOrString::Bool(plant.info.auto_water),
            current_location: plant.info.location.clone(),
            obtained: plant.info.obtained.format(&date_format).to_string(),
            origin: plant.info.origin.clone(),
            plant_health: plant
                .growth
                .last()
                .map(|gr| gr.health)
                .unwrap_or(3)
                .to_string(),
            plant_notes: plant.info.notes.clone(),
        }
    }
}*/
