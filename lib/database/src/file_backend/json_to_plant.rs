use super::{
    errors::{AccessType, Error, FSError},
    load_csv::{load_activities, load_growth},
    load_json::{load_plant_jsons, load_species},
};
use crate::database_manager::{BoolOrString, PlantJSON};
use chrono::NaiveDate;
use plants::{
    growth_item::GrowthItem,
    log_item::LogItem,
    named::Named,
    plant::{Plant, PlantImage},
    species::Species,
};
use std::fs;

impl From<(&Plant, String)> for PlantJSON {
    fn from((plant, date_format): (&Plant, String)) -> PlantJSON {
        PlantJSON {
            plant_name: plant.name.clone(),
            species_name: plant
                .species
                .clone()
                .map(|sp| sp.name)
                .unwrap_or("".to_owned()),
            auto_watering: BoolOrString::Bool(plant.auto_water),
            current_location: plant.location.clone(),
            obtained: plant.obtained.format(&date_format).to_string(),
            origin: plant.origin.clone(),
            plant_health: plant
                .growth
                .last()
                .map(|gr| gr.health)
                .unwrap_or(3)
                .to_string(),
            plant_notes: plant.notes.clone(),
        }
    }
}

struct PlantInfo {
    plant: PlantJSON,
    species: Option<Species>,
    logs: Vec<LogItem>,
    growth: Vec<GrowthItem>,
    date_format: String,
    images: Vec<PlantImage>,
}

impl Named for PlantJSON {
    fn get_name(&self) -> String {
        self.plant_name.clone()
    }
}

impl From<(&Plant, String)> for PlantInfo {
    fn from((plant, date_format): (&Plant, String)) -> PlantInfo {
        PlantInfo {
            plant: (plant, date_format.clone()).into(),
            species: plant.species.clone(),
            logs: plant.activities.clone(),
            growth: plant.growth.clone(),
            date_format,
            images: plant.images.clone(),
        }
    }
}

impl TryInto<Plant> for PlantInfo {
    type Error = Error;
    fn try_into(self) -> Result<Plant, Self::Error> {
        let new_obtained = NaiveDate::parse_from_str(&self.plant.obtained, &self.date_format)?;
        let new_autowater = self.plant.auto_watering.try_into()?;
        let species = self.species;
        Ok(Plant {
            name: self.plant.plant_name,
            species,
            location: self.plant.current_location,
            origin: self.plant.origin,
            obtained: new_obtained,
            auto_water: new_autowater,
            notes: self.plant.plant_notes,
            activities: self.logs,
            growth: self.growth,
            images: self.images,
        })
    }
}

pub fn load_plants(
    plants_dir: &str,
    species_dir: &str,
    activity_file: &str,
    growth_file: &str,
    date_format: &str,
) -> Result<Vec<Plant>, Error> {
    log::info!("Loading plants");
    let plant_jsons: Vec<PlantJSON> = load_plant_jsons(plants_dir)?;
    let species = load_species(species_dir)?;
    let logs = load_activities(activity_file)?;
    let growth = load_growth(growth_file)?;
    let mut plants = vec![];
    for plant_json in plant_jsons.iter() {
        log::info!("Loading plant {}", plant_json.plant_name);
        let species_plant = species
            .iter()
            .find(|sp| {
                sp.name.to_lowercase().trim() == plant_json.species_name.to_lowercase().trim()
            })
            .cloned();
        if species_plant.is_none() {
            log::warn!("Could not find species for plant {}", plant_json.plant_name);
        }
        let plant_logs: Vec<LogItem> = logs
            .iter()
            .filter(|log| log.plant == plant_json.plant_name)
            .cloned()
            .collect();
        if plant_logs.is_empty() {
            log::warn!("No logs for plant {}", plant_json.plant_name);
        }
        let mut plant_growth: Vec<GrowthItem> = growth
            .iter()
            .filter(|growth| growth.plant == plant_json.plant_name)
            .cloned()
            .collect();
        let last_health = plant_json.plant_health.parse::<i32>()?;
        let mut last_growth =
            plant_growth
                .pop()
                .ok_or(Error::PlantError(plants::errors::Error::GrowthError(
                    plant_json.plant_name.clone(),
                )))?;
        let images = load_images("html_out/img/plants", &plant_json.plant_name)?;
        if images.is_empty() {
            log::warn!("No images for plant {}", plant_json.plant_name);
        }
        last_growth.health = last_health;
        plant_growth.push(last_growth);
        let new_plant = PlantInfo {
            plant: plant_json.clone(),
            species: species_plant,
            logs: plant_logs.clone(),
            growth: plant_growth.clone(),
            date_format: date_format.to_owned(),
            images,
        }
        .try_into()?;
        plants.push(new_plant);
    }
    Ok(plants)
}

pub fn load_images(image_dir: &str, plant_name: &str) -> Result<Vec<PlantImage>, Error> {
    let mut plant_images = vec![];
    let dir_files = fs::read_dir(image_dir).map_err(|err| {
        <FSError as Into<Error>>::into(FSError {
            file_name: image_dir.to_owned(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })
    })?;
    for dir_file in dir_files {
        let dir_file = dir_file.map_err(|err| {
            <FSError as Into<Error>>::into(FSError {
                file_name: image_dir.to_owned(),
                err_msg: err.to_string(),
                access: AccessType::Read,
            })
        })?;
        let path = dir_file.path();
        let file_base = path.file_name().ok_or(Error::FSError(FSError {
            file_name: image_dir.to_owned(),
            err_msg: "Could not find path".to_owned(),
            access: AccessType::Read,
        }))?;
        let file_name = file_base.to_str().ok_or(Error::FSError(FSError {
            file_name: image_dir.to_owned(),
            err_msg: "Could not get name as string".to_owned(),
            access: AccessType::Read,
        }))?;
        if file_name.contains(plant_name) {
            let file_end = file_name.split('_').last().ok_or(Error::FSError(FSError {
                file_name: file_name.to_owned(),
                err_msg: "Filename did not contain date".to_owned(),
                access: AccessType::Read,
            }))?;
            let parts = file_end.split('.').collect::<Vec<&str>>();

            let date_str = parts.first().ok_or(Error::FSError(FSError {
                file_name: file_name.to_owned(),
                err_msg: "Filename did not contain date".to_owned(),
                access: AccessType::Read,
            }))?;
            let created = NaiveDate::parse_from_str(date_str, "%d%m%Y")?;
            plant_images.push((created, file_name.to_owned()))
        }
    }
    Ok(plant_images)
}
