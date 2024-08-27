use super::{
    errors::{Error, IOErr, ParseError},
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
use std::{ffi::OsString, fs, path::PathBuf};

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
        log::info!("Found species {:?} for {}", species_plant, plant_info.name);
        plant_info.species = species_plant;

        let plant_logs: Vec<LogItem> = logs
            .iter()
            .filter(|log| log.plant == plant_info.name)
            .cloned()
            .collect();
        log::info!("Loaded logs for plant {}", plant_info.name);
        if plant_logs.is_empty() {
            log::warn!("No logs for plant {}", plant_info.name);
        }

        let plant_growth: Vec<GrowthItem> = growth
            .iter()
            .filter(|growth| growth.plant == plant_info.name)
            .cloned()
            .collect();
        log::info!("Loaded growth for plant {}", plant_info.name);

        let img_dir = plants_dir.join(plant_info.name.replace(' ', ""));
        let images = load_images(&img_dir)?;
        log::info!("Loaded images for plant {}", plant_info.name);
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

pub fn load_images(image_dir: &PathBuf) -> Result<Vec<PlantImage>, Error> {
    let mut plant_images = vec![];
    let dir_files = fs::read_dir(image_dir)?;
    for dir_file in dir_files {
        let dir_file = dir_file?;
        let path = dir_file.path();
        if path.extension() != Some(&OsString::from("jpg")) {
            continue;
        }

        let stem = path.file_stem().ok_or(IOErr {
            kind: "Get File Stem".to_owned(),
        })?;
        let stem_str = stem.to_str().ok_or(IOErr {
            kind: "Convert from OS String".to_owned(),
        })?;

        let created = NaiveDate::parse_from_str(stem_str, "%d%m%Y").map_err(|_| ParseError {
            ty: "Date".to_owned(),
            input: stem_str.to_owned().to_owned(),
        })?;
        let file_path = path.parent().ok_or(IOErr {
            kind: "Get Path Parent".to_owned(),
        })?;
        let image = PlantImage {
            created,
            file_name: stem_str.to_owned() + ".jpg",
            file_path: file_path.to_path_buf(),
        };
        plant_images.push(image)
    }

    Ok(plant_images)
}
