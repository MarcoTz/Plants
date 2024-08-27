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
