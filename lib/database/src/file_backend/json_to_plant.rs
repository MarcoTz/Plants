use super::{
    errors::{AccessType, Error, FSError},
    load_csv::{load_activities, load_growth},
    load_json::{load_plant_infos, load_species},
};
use chrono::NaiveDate;
use plants::{
    growth_item::GrowthItem,
    log_item::LogItem,
    named::Named,
    plant::{Plant, PlantImage, PlantSpecies},
};
use std::{fs, path::PathBuf};

pub fn load_plants(
    plants_dir: &PathBuf,
    species_dir: &PathBuf,
    activity_file: &PathBuf,
    growth_file: &PathBuf,
) -> Result<Vec<Plant>, Error> {
    log::info!("Loading plants");
    let mut plant_infos = load_plant_infos(plants_dir)?;
    let species = load_species(species_dir)?;
    let logs = load_activities(activity_file)?;
    let growth = load_growth(growth_file)?;
    let mut plants = vec![];
    for plant_info in plant_infos.iter_mut() {
        log::info!("Loading plant {}", plant_info.name);
        let species_plant = species
            .iter()
            .find(|sp| sp.name.to_lowercase().trim() == plant_info.species.get_name())
            .cloned()
            .map(|sp| PlantSpecies::Species(Box::new(sp)))
            .unwrap_or(PlantSpecies::Other(plant_info.name.clone()));
        plant_info.species = species_plant;

        let plant_logs: Vec<LogItem> = logs
            .iter()
            .filter(|log| log.plant == plant_info.name)
            .cloned()
            .collect();
        if plant_logs.is_empty() {
            log::warn!("No logs for plant {}", plant_info.name);
        }

        let plant_growth: Vec<GrowthItem> = growth
            .iter()
            .filter(|growth| growth.plant == plant_info.name)
            .cloned()
            .collect();

        let img_dir = plants_dir.join(plant_info.name.replace(' ', ""));
        let images = load_images(&img_dir, &plant_info.name)?;
        if images.is_empty() {
            log::warn!("No images for plant {}", plant_info.name);
        }

        let new_plant = Plant {
            info: plant_info.clone(),
            activities: plant_logs.clone(),
            growth: plant_growth.clone(),
            images,
        };
        plants.push(new_plant);
    }
    Ok(plants)
}

pub fn load_images(image_dir: &PathBuf, plant_name: &str) -> Result<Vec<PlantImage>, Error> {
    let mut plant_images = vec![];
    let dir_files = fs::read_dir(image_dir).map_err(|err| {
        <FSError as Into<Error>>::into(FSError {
            path: image_dir.clone(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })
    })?;
    for dir_file in dir_files {
        let dir_file = dir_file.map_err(|err| FSError {
            path: image_dir.clone(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })?;
        let path = dir_file.path();
        let file_base = path.file_name().ok_or(FSError {
            path: path.clone(),
            err_msg: "Could not find path".to_owned(),
            access: AccessType::Read,
        })?;
        let file_name = file_base.to_str().ok_or(FSError {
            path: image_dir.clone(),
            err_msg: "Could not get name as string".to_owned(),
            access: AccessType::Read,
        })?;
        if file_name.contains(plant_name) {
            let file_end = file_name.split('_').last().ok_or(FSError {
                path: path.clone(),
                err_msg: "Filename did not contain date".to_owned(),
                access: AccessType::Read,
            })?;
            let parts = file_end.split('.').collect::<Vec<&str>>();

            let date_str = parts.first().ok_or(FSError {
                path: path.clone(),
                err_msg: "Filename did not contain date".to_owned(),
                access: AccessType::Read,
            })?;
            let created = NaiveDate::parse_from_str(date_str, "%d%m%Y")?;
            plant_images.push((created, file_name.to_owned()))
        }
    }
    Ok(plant_images)
}
