use super::errors::{Error, SerializeError};
use plants::{plant::PlantInfo, species::Species};
use serde::de::DeserializeOwned;
use std::{ffi::OsString, fs, fs::DirEntry, path::PathBuf};

pub fn load_json<T: DeserializeOwned>(file_name: &PathBuf) -> Result<T, Error> {
    log::info!("Loading JSON {:?}", file_name);
    let file_contents = fs::read_to_string(file_name)?;
    let res = serde_json::from_str(&file_contents).map_err(|err| SerializeError {
        path: file_name.clone(),
        err_msg: err.to_string(),
    })?;
    Ok(res)
}

pub fn load_dir<T: DeserializeOwned>(dir_path: &PathBuf) -> Result<Vec<T>, Error> {
    log::info!("Loading JSON from dir {:?}", dir_path);
    let mut struct_list = vec![];
    for dir_entry in fs::read_dir(dir_path)? {
        let entry = dir_entry?;
        let entry_contents = fs::read_dir(entry.path())?;
        let content_paths = entry_contents
            .map(|x| x.map_err(|err| err.into()))
            .collect::<Result<Vec<DirEntry>, Error>>()?;
        let json_files = content_paths
            .into_iter()
            .filter(|entry| entry.path().extension() == Some(&OsString::from("json")));

        for json_file in json_files {
            let json_contents = load_json(&json_file.path())?;
            struct_list.push(json_contents);
        }
    }
    Ok(struct_list)
}

pub fn load_plant_infos(plants_dir: &PathBuf) -> Result<Vec<PlantInfo>, Error> {
    load_dir(plants_dir)
}

pub fn load_species(species_dir: &PathBuf) -> Result<Vec<Species>, Error> {
    let species: Vec<Species> = load_dir(species_dir)?;
    Ok(species)
}

#[cfg(test)]
mod load_json_tests {

    use super::{load_dir, load_json, load_plant_infos, load_species};
    use crate::file_backend::test_common::{
        dummy_plant1, dummy_plant2, dummy_species, example_json1, DummyJSON, DUMMY_PLANT_PATH,
        DUMMY_SPECIES_PATH, FILE_DOES_NOT_EXIST, JSON_DUMMY, JSON_DUMMY_DESERIALIZE,
        JSON_DUMMY_DIR, JSON_DUMMY_NO_SUBDIR,
    };
    use plants::plant::{PlantLocation, PlantSpecies};
    use std::path::PathBuf;

    #[test]
    fn load_dummy_json() {
        let result = load_json::<DummyJSON>(&PathBuf::from(&JSON_DUMMY)).unwrap();
        let expected = example_json1();
        assert_eq!(result, expected)
    }

    #[test]
    fn load_json_no_file() {
        let result = load_json::<DummyJSON>(&PathBuf::from(&FILE_DOES_NOT_EXIST));
        assert!(result.is_err())
    }

    #[test]
    fn load_json_serialize() {
        let result = load_json::<DummyJSON>(&PathBuf::from(&JSON_DUMMY_DESERIALIZE));
        assert!(result.is_err())
    }

    #[test]
    fn load_json_dir() {
        let result = load_dir::<DummyJSON>(&PathBuf::from(&JSON_DUMMY_DIR)).unwrap();
        let expected = vec![
            example_json1(),
            example_json1(),
            example_json1(),
            example_json1(),
            example_json1(),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn load_json_no_subdir() {
        let result = load_dir::<DummyJSON>(&PathBuf::from(&JSON_DUMMY_NO_SUBDIR));
        assert!(result.is_err())
    }

    #[test]
    fn load_json_dir_bad_json() {
        let result = load_dir::<DummyJSON>(&PathBuf::from(&JSON_DUMMY_NO_SUBDIR));
        assert!(result.is_err())
    }

    #[test]
    fn load_plant_jsons() {
        let mut result = load_plant_infos(&PathBuf::from(&DUMMY_PLANT_PATH)).unwrap();
        result.sort_by(|plant1, plant2| plant1.name.cmp(&plant2.name));
        let mut plant1 = dummy_plant1().info;
        plant1.species = PlantSpecies::Other("test species".to_owned());
        plant1.location = PlantLocation::Other("test location".to_owned());
        let mut expected = vec![plant1, dummy_plant2().info];
        expected.sort_by(|plant1, plant2| plant1.name.cmp(&plant2.name));
        assert_eq!(result, expected)
    }

    #[test]
    fn load_species_jsons() {
        let result = load_species(&PathBuf::from(&DUMMY_SPECIES_PATH)).unwrap();
        let expected = vec![dummy_species()];
        assert_eq!(result, expected)
    }
}
