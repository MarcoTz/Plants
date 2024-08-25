use super::errors::{AccessType, Error, FSError, SerializeError};
use plants::{plant::PlantInfo, species::Species};
use serde::de::DeserializeOwned;
use std::{fs, path::PathBuf};

pub fn load_json<T: DeserializeOwned>(file_name: &PathBuf) -> Result<T, Error> {
    log::info!("Loading JSON {:?}", file_name);
    let file_contents = fs::read_to_string(file_name).map_err(|err| FSError {
        path: file_name.clone(),
        err_msg: err.to_string(),
        access: AccessType::Read,
    })?;
    let res = serde_json::from_str(&file_contents).map_err(|err| SerializeError {
        path: file_name.clone(),
        err_msg: err.to_string(),
        access: AccessType::Write,
    })?;
    Ok(res)
}

pub fn load_dir<T: DeserializeOwned>(dir_path: &PathBuf) -> Result<Vec<T>, Error> {
    log::info!("Loading JSON from dir {:?}", dir_path);
    let mut struct_list = vec![];
    for dir_entry in fs::read_dir(dir_path).map_err(|err| {
        <FSError as Into<Error>>::into(FSError {
            path: dir_path.clone(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })
    })? {
        let entry = dir_entry.map_err(|err| FSError {
            path: dir_path.clone(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })?;
        let json_contents: T = load_json(&entry.path())?;
        struct_list.push(json_contents);
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
