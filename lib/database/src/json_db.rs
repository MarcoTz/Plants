use super::errors::DBError;
use super::json_to_plant::PlantJSONOld;
use super::json_to_species::SpeciesJSONOld;
use serde::de::DeserializeOwned;
use std::fs;

const PLANTS_DIR: &str = "data/Plants";
const SPECIES_DIR: &str = "data/PlantSpecies";

fn load_json<T: DeserializeOwned>(file_name: &str) -> Result<T, DBError> {
    println!("Reading file {}", file_name);
    let file_contents = fs::read_to_string(file_name)?;
    let res = serde_json::from_str(&file_contents)?;
    Ok(res)
}

pub fn load_dir<T: DeserializeOwned>(dir_path: &str) -> Result<Vec<T>, DBError> {
    let mut struct_list = vec![];
    for dir_entry in fs::read_dir(dir_path)? {
        let entry = dir_entry?;
        let m_path = entry.path();
        let path_str = m_path
            .to_str()
            .to_owned()
            .ok_or(DBError::PathError("no clue".to_owned()))?;
        let json_contents: T = load_json(path_str)?;
        struct_list.push(json_contents);
    }
    Ok(struct_list)
}
pub fn load_plants() -> Result<Vec<PlantJSONOld>, DBError> {
    println!("reading plants");
    load_dir(PLANTS_DIR)
}

pub fn load_species() -> Result<Vec<SpeciesJSONOld>, DBError> {
    println!("reading species");
    load_dir(SPECIES_DIR)
}
