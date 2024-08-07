use super::errors::{AccessType, Error, FSError, SerializeError};
use super::json_to_plant::PlantJSON;
use super::json_to_species::SpeciesJSON;
use plants::plant::Plant;
use plants::species::Species;
use serde::de::DeserializeOwned;
use std::fs;

fn load_json<T: DeserializeOwned>(file_name: &str) -> Result<T, Error> {
    let file_contents = fs::read_to_string(file_name).map_err(|err| {
        <FSError as Into<Error>>::into(FSError {
            file_name: file_name.to_owned(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })
    })?;
    let res = serde_json::from_str(&file_contents).map_err(|err| {
        <SerializeError as Into<Error>>::into(SerializeError {
            out_path: file_name.to_owned(),
            err_msg: err.to_string(),
            access: AccessType::Write,
        })
    })?;
    Ok(res)
}

pub fn load_dir<T: DeserializeOwned>(dir_path: &str) -> Result<Vec<T>, Error> {
    let mut struct_list = vec![];
    for dir_entry in fs::read_dir(dir_path).map_err(|err| {
        <FSError as Into<Error>>::into(FSError {
            file_name: dir_path.to_owned(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })
    })? {
        let entry = dir_entry.map_err(|err| {
            <FSError as Into<Error>>::into(FSError {
                file_name: dir_path.to_owned(),
                err_msg: err.to_string(),
                access: AccessType::Read,
            })
        })?;
        let m_path = entry.path();
        let path_str = m_path.to_str().to_owned().ok_or(Error::FSError(FSError {
            file_name: dir_path.to_owned(),
            err_msg: "Could not find path".to_owned(),
            access: AccessType::Read,
        }))?;
        let json_contents: T = load_json(path_str)?;
        struct_list.push(json_contents);
    }
    Ok(struct_list)
}
pub fn load_plants(plants_dir: &str) -> Result<Vec<PlantJSON>, Error> {
    let plants_old: Vec<PlantJSON> = load_dir(plants_dir)?;
    Ok(plants_old)
}

pub fn load_species(species_dir: &str) -> Result<Vec<Species>, Error> {
    let species_old: Vec<SpeciesJSON> = load_dir(species_dir)?;
    let species_new = species_old
        .iter()
        .cloned()
        .map(<SpeciesJSON as TryInto<Species>>::try_into)
        .collect::<Result<Vec<Species>, Error>>()?;
    Ok(species_new)
}
