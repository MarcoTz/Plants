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
    println!("loading images from dir {image_dir:?}");
    let mut plant_images = vec![];
    let dir_files = fs::read_dir(image_dir)?;
    for dir_file in dir_files {
        let dir_file = dir_file?;
        let path = dir_file.path();
        if path.extension() != Some(&OsString::from("jpg")) {
            continue;
        }
        println!("Found jpg file {path:?}");
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
    use crate::file_backend::test_common::dummy_date;
    use plants::{
        growth_item::GrowthItem,
        log_item::LogItem,
        plant::{Plant, PlantImage, PlantInfo, PlantLocation, PlantSpecies},
        species::{Species, SunlightRequirement},
    };
    use std::path::PathBuf;

    const DUMMY_PLANT_PATH: &str = "../../testing/plants/";
    const DUMMY_SPECIES_PATH: &str = "../../testing/species/";
    const DUMMY_ACTIVITY_FILE: &str = "../../testing/Logs/Activities.csv";
    const DUMMY_GROWTH_FILE: &str = "../../testing/Logs/Growth.csv";

    fn dummy_species() -> Species {
        Species {
            name: "test species".to_owned(),
            scientific_name: "testing name".to_owned(),
            genus: "testing genus".to_owned(),
            family: "testing family".to_owned(),
            sunlight: SunlightRequirement::Direct,
            temp_min: 0.0,
            temp_max: 0.0,
            opt_temp_min: 0.0,
            opt_temp_max: 0.0,
            planting_distance: Some(0.0),
            ph_min: 0.0,
            ph_max: 0.0,
            watering_notes: vec!["".to_owned()],
            avg_watering_days: Some(0),
            fertilizing_notes: vec!["".to_owned()],
            avg_fertilizing_days: Some(0),
            pruning_notes: vec!["".to_owned()],
            companions: vec!["".to_owned()],
            additional_notes: vec![],
        }
    }

    fn dummy_plant1() -> Plant {
        Plant {
            info: PlantInfo {
                name: "Dummy1".to_owned(),
                species: PlantSpecies::Species(Box::new(dummy_species())),
                location: PlantLocation::Other("test location".to_owned()),
                origin: "test origin".to_owned(),
                obtained: dummy_date(),
                auto_water: true,
                notes: vec![],
            },
            growth: vec![GrowthItem {
                plant: "Dummy1".to_owned(),
                date: dummy_date(),
                height_cm: 10.0,
                width_cm: 10.0,
                note: None,
                health: 3,
            }],
            activities: vec![LogItem {
                activity: "Watering".to_owned(),
                date: dummy_date(),
                plant: "Dummy1".to_owned(),
                note: None,
            }],
            images: vec![PlantImage {
                created: dummy_date(),
                file_name: "01011970.jpg".to_owned(),
                file_path: PathBuf::from("../../testing/plants/Dummy1"),
            }],
        }
    }

    fn dummy_plant2() -> Plant {
        Plant {
            info: PlantInfo {
                name: "Dummy2".to_owned(),
                species: PlantSpecies::Other("a different species".to_owned()),
                location: PlantLocation::Other("test location".to_owned()),
                origin: "test origin".to_owned(),
                obtained: dummy_date(),
                auto_water: true,
                notes: vec![],
            },
            growth: vec![],
            activities: vec![],
            images: vec![],
        }
    }

    #[test]
    fn load_dummy_plant() {
        let mut result = load_plants(
            &PathBuf::from(&DUMMY_PLANT_PATH),
            &PathBuf::from(&DUMMY_SPECIES_PATH),
            &PathBuf::from(&DUMMY_ACTIVITY_FILE),
            &PathBuf::from(&DUMMY_GROWTH_FILE),
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
            &PathBuf::from("../../testing/notadir"),
            &PathBuf::from(&DUMMY_SPECIES_PATH),
            &PathBuf::from(&DUMMY_ACTIVITY_FILE),
            &PathBuf::from(&DUMMY_GROWTH_FILE),
        );
        assert!(result.is_err())
    }

    #[test]
    fn load_bad_species_dir() {
        let result = load_plants(
            &PathBuf::from(&DUMMY_PLANT_PATH),
            &PathBuf::from("../../testing/notadir"),
            &PathBuf::from(&DUMMY_ACTIVITY_FILE),
            &PathBuf::from(&DUMMY_GROWTH_FILE),
        );
        assert!(result.is_err())
    }

    #[test]
    fn load_bad_activity_file() {
        let result = load_plants(
            &PathBuf::from(&DUMMY_PLANT_PATH),
            &PathBuf::from(&DUMMY_SPECIES_PATH),
            &PathBuf::from("../../testing/notafile"),
            &PathBuf::from(&DUMMY_GROWTH_FILE),
        );
        assert!(result.is_err())
    }

    #[test]
    fn load_bad_growth_file() {
        let result = load_plants(
            &PathBuf::from(&DUMMY_PLANT_PATH),
            &PathBuf::from(&DUMMY_SPECIES_PATH),
            &PathBuf::from(&DUMMY_ACTIVITY_FILE),
            &PathBuf::from("../../testing/notafile"),
        );
        assert!(result.is_err())
    }

    #[test]
    fn load_images_no_date() {
        let result = load_images(&PathBuf::from("../../testing/"));
        assert!(result.is_err())
    }

    #[test]
    fn load_images_bad_dir() {
        let result = load_images(&PathBuf::from("../../testing/notadir"));
        assert!(result.is_err())
    }
}
