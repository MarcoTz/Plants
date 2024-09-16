use super::errors::{Error, SerializeError};
use plants::species::Species;
use plants::{named::Named, plant::PlantInfo};
use serde::Serialize;
use std::io::Write;
use std::{
    fs::{create_dir_all, File},
    path::{Path, PathBuf},
};

pub fn write_json<T: Serialize>(item: T, out_filepath: &PathBuf) -> Result<(), Error> {
    log::info!("Writing JSON {:?}", out_filepath);
    let serialized = serde_json::to_string(&item).map_err(|err| SerializeError {
        path: out_filepath.clone(),
        err_msg: err.to_string(),
    })?;
    let mut out_file = File::create(out_filepath)?;
    out_file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn write_vec<T: Serialize + Named>(items: Vec<T>, out_path: &PathBuf) -> Result<(), Error> {
    for item in items.iter() {
        let item_name = &item.get_name().replace(' ', "");
        let file_name = format!("{item_name}.json");
        let out_dir = Path::new(out_path).join(item_name);
        create_dir_all(out_dir.clone())?;
        let out_path = out_dir.join(file_name);
        write_json(item, &out_path)?;
    }
    Ok(())
}

pub fn write_plants(plants: Vec<PlantInfo>, plant_dir: &PathBuf) -> Result<(), Error> {
    write_vec(plants, plant_dir)
}

pub fn write_species(species: Vec<Species>, species_dir: &PathBuf) -> Result<(), Error> {
    write_vec(species, species_dir)
}

#[cfg(test)]
mod write_json_tests {

    use super::{write_json, write_plants, write_species, write_vec};
    use crate::file_backend::{
        load_json::load_json,
        test_common::{
            dummy_plant1, dummy_plant2, dummy_species, example_json1, example_json2, DummyJSON,
            JSON_DUMMY_OUT, JSON_DUMMY_OUT_DIR, JSON_DUMMY_PLANT_OUT_DIR,
            JSON_DUMMY_SPECIES_OUT_DIR,
        },
    };
    use plants::{
        named::Named,
        plant::{PlantInfo, PlantLocation, PlantSpecies},
        species::Species,
    };
    use std::path::PathBuf;

    #[test]
    fn write_dummy_json() {
        let values = example_json1();
        let json_file = PathBuf::from(JSON_DUMMY_OUT);
        write_json(values, &json_file).unwrap();
        let result = load_json::<DummyJSON>(&json_file).unwrap();
        let expected = example_json1();
        assert_eq!(result, expected);
        std::fs::remove_file(json_file.clone()).unwrap();
        assert!(!json_file.exists())
    }

    #[test]
    fn write_dummy_json_vec() {
        let value1 = example_json1();
        let value2 = example_json2();
        let json_dir = PathBuf::from(JSON_DUMMY_OUT_DIR);
        write_vec(vec![value1.clone(), value2.clone()], &json_dir).unwrap();
        let name1 = value1.get_name().replace(' ', "");
        let name2 = value2.get_name().replace(' ', "");
        let dir1 = json_dir.join(name1.clone());
        let dir2 = json_dir.join(name2.clone());
        let file1 = json_dir.join(name1.clone()).join(name1 + ".json");
        let file2 = json_dir.join(name2.clone()).join(name2 + ".json");
        let result1 = load_json::<DummyJSON>(&file1).unwrap();
        let result2 = load_json::<DummyJSON>(&file2).unwrap();

        assert_eq!(result1, value1);
        assert_eq!(result2, value2);
        assert!(dir1.exists());
        assert!(dir2.exists());
        assert!(file1.exists());
        assert!(file2.exists());
        std::fs::remove_dir_all(dir1.clone()).unwrap();
        std::fs::remove_dir_all(dir2.clone()).unwrap();
        assert!(!dir1.exists());
        assert!(!dir2.exists());
    }

    #[test]
    fn write_dummy_plants() {
        let mut value1 = dummy_plant1().info;
        let value2 = dummy_plant2().info;
        let json_dir = PathBuf::from(JSON_DUMMY_PLANT_OUT_DIR);
        write_plants(vec![value1.clone(), value2.clone()], &json_dir).unwrap();
        let name1 = value1.get_name().replace(' ', "");
        let name2 = value2.get_name().replace(' ', "");
        let dir1 = json_dir.join(name1.clone());
        let dir2 = json_dir.join(name2.clone());
        let file1 = dir1.join(name1 + ".json");
        let file2 = dir2.join(name2 + ".json");
        let result1 = load_json::<PlantInfo>(&file1).unwrap();
        let result2 = load_json::<PlantInfo>(&file2).unwrap();
        value1.species = PlantSpecies::Other(value1.species.get_name());
        value1.location = PlantLocation::Other(value1.location.get_name());

        assert_eq!(result1, value1);
        assert_eq!(result2, value2);
        assert!(dir1.exists());
        assert!(dir2.exists());
        assert!(file1.exists());
        assert!(file2.exists());
        std::fs::remove_dir_all(dir1.clone()).unwrap();
        std::fs::remove_dir_all(dir2.clone()).unwrap();
        assert!(!dir1.exists());
        assert!(!dir2.exists());
    }

    #[test]
    fn write_dummy_species() {
        let value = dummy_species();
        let json_dir = PathBuf::from(JSON_DUMMY_SPECIES_OUT_DIR);
        write_species(vec![value.clone()], &json_dir).unwrap();
        let name = value.get_name().replace(' ', "");
        let dir = json_dir.join(name.clone());
        let file = dir.join(name + ".json");
        let result = load_json::<Species>(&file).unwrap();

        assert_eq!(result, value);
        assert!(dir.exists());
        assert!(file.exists());
        std::fs::remove_dir_all(dir.clone()).unwrap();
        assert!(!dir.exists())
    }
}
