use super::{
    errors::{Error, IOErr, ParseError},
    load_csv::{load_activities, load_growth, load_locations},
    load_json::{load_plant_infos, load_species},
};
use chrono::NaiveDate;
use plants::{
    growth_item::GrowthItem,
    log_item::LogItem,
    named::Named,
    plant::{Plant, PlantImage, PlantLocation, PlantSpecies},
};
use std::{ffi::OsString, fs, path::PathBuf};

pub fn load_plants(
    plants_dir: &PathBuf,
    species_dir: &PathBuf,
    activity_file: &PathBuf,
    growth_file: &PathBuf,
    locations_file: &PathBuf,
) -> Result<Vec<Plant>, Error> {
    log::info!("Loading plants");
    let mut plant_infos = load_plant_infos(plants_dir)?;
    let species = load_species(species_dir)?;
    let locations = load_locations(locations_file)?;
    let logs = load_activities(activity_file)?;
    let growth = load_growth(growth_file)?;
    let mut plants = vec![];
    for plant_info in plant_infos.iter_mut() {
        log::info!("Loading plant {}", plant_info.name);
        let species_plant = species
            .iter()
            .find(|sp| sp.name.trim() == plant_info.species.get_name())
            .cloned()
            .map(|sp| PlantSpecies::Species(Box::new(sp)))
            .unwrap_or(PlantSpecies::Other(plant_info.species.get_name()));
        if let PlantSpecies::Species(_) = species_plant {
            log::info!(
                "Found species {} for {}",
                species_plant.get_name(),
                plant_info.name
            );
        } else {
            log::warn!(
                "Could not find species {} for {}",
                species_plant.get_name(),
                plant_info.name
            );
        }

        let location_plant = locations
            .iter()
            .find(|loc| loc.get_name() == plant_info.location.get_name())
            .cloned()
            .map(|loc| PlantLocation::Location(Box::new(loc)))
            .unwrap_or(PlantLocation::Other(plant_info.location.get_name()));

        plant_info.location = location_plant;

        if let PlantLocation::Location(_) = plant_info.location {
            log::info!(
                "Found location {} for {}",
                plant_info.location.get_name(),
                plant_info.name
            );
        } else {
            log::warn!(
                "Could not find location {} for {}",
                plant_info.location.get_name(),
                plant_info.name
            );
        }

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
    if !image_dir.exists() {
        fs::create_dir_all(image_dir)?;
    }
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

#[cfg(test)]
mod json_to_plant_tests {
    use super::{load_images, load_plants};
    use crate::file_backend::test_common::{
        dummy_plant1, dummy_plant2, ACTIVITIES_DUMMY, DUMMY_PLANT_PATH, DUMMY_SPECIES_PATH,
        FILE_DOES_NOT_EXIST, GROWTH_DUMMY, LOCATIONS_DUMMY, TESTING_BASE,
    };
    use std::path::PathBuf;

    #[test]
    fn load_dummy_plants() {
        let mut result = load_plants(
            &PathBuf::from(&DUMMY_PLANT_PATH),
            &PathBuf::from(&DUMMY_SPECIES_PATH),
            &PathBuf::from(&ACTIVITIES_DUMMY),
            &PathBuf::from(&GROWTH_DUMMY),
            &PathBuf::from(&LOCATIONS_DUMMY),
        )
        .unwrap();
        result.sort_by(|plant1, plant2| plant1.info.name.cmp(&plant2.info.name));
        let mut expected = vec![dummy_plant1(), dummy_plant2()];
        expected.sort_by(|plant1, plant2| plant1.info.name.cmp(&plant2.info.name));
        assert_eq!(result, expected)
    }

    #[test]
    fn load_bad_plant_dir() {
        let result = load_plants(
            &PathBuf::from(&FILE_DOES_NOT_EXIST),
            &PathBuf::from(&DUMMY_SPECIES_PATH),
            &PathBuf::from(&ACTIVITIES_DUMMY),
            &PathBuf::from(&GROWTH_DUMMY),
            &PathBuf::from(&LOCATIONS_DUMMY),
        );
        assert!(result.is_err())
    }

    #[test]
    fn load_bad_species_dir() {
        let result = load_plants(
            &PathBuf::from(&DUMMY_PLANT_PATH),
            &PathBuf::from(&FILE_DOES_NOT_EXIST),
            &PathBuf::from(&ACTIVITIES_DUMMY),
            &PathBuf::from(&GROWTH_DUMMY),
            &PathBuf::from(&LOCATIONS_DUMMY),
        );
        assert!(result.is_err())
    }

    #[test]
    fn load_bad_activity_file() {
        let result = load_plants(
            &PathBuf::from(&DUMMY_PLANT_PATH),
            &PathBuf::from(&DUMMY_SPECIES_PATH),
            &PathBuf::from(&FILE_DOES_NOT_EXIST),
            &PathBuf::from(&GROWTH_DUMMY),
            &PathBuf::from(&LOCATIONS_DUMMY),
        );
        assert!(result.is_err())
    }

    #[test]
    fn load_bad_growth_file() {
        let result = load_plants(
            &PathBuf::from(&DUMMY_PLANT_PATH),
            &PathBuf::from(&DUMMY_SPECIES_PATH),
            &PathBuf::from(&ACTIVITIES_DUMMY),
            &PathBuf::from(&FILE_DOES_NOT_EXIST),
            &PathBuf::from(&LOCATIONS_DUMMY),
        );
        assert!(result.is_err())
    }

    #[test]
    fn load_bad_locations_file() {
        let result = load_plants(
            &PathBuf::from(&DUMMY_PLANT_PATH),
            &PathBuf::from(&DUMMY_SPECIES_PATH),
            &PathBuf::from(&ACTIVITIES_DUMMY),
            &PathBuf::from(&GROWTH_DUMMY),
            &PathBuf::from(&FILE_DOES_NOT_EXIST),
        );
        assert!(result.is_err())
    }

    #[test]
    fn load_images_no_date() {
        let result = load_images(&PathBuf::from(&TESTING_BASE));
        assert!(result.is_err())
    }

    #[test]
    fn load_images_bad_dir() {
        let result = load_images(&PathBuf::from(&FILE_DOES_NOT_EXIST));
        assert!(result.is_err())
    }
}
