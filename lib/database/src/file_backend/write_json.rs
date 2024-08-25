use super::errors::{AccessType, Error, FSError, SerializeError};
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
        access: AccessType::Write,
    })?;
    let mut out_file = File::create(out_filepath).map_err(|err| FSError {
        path: out_filepath.clone(),
        err_msg: err.to_string(),
        access: AccessType::Write,
    })?;
    out_file
        .write_all(serialized.as_bytes())
        .map_err(|err| FSError {
            path: out_filepath.clone(),
            err_msg: err.to_string(),
            access: AccessType::Write,
        })?;
    Ok(())
}

pub fn write_vec<T: Serialize + Named>(items: Vec<T>, out_path: &PathBuf) -> Result<(), Error> {
    for item in items.iter() {
        let item_name = &item.get_name();
        let file_name = format!("{item_name}.json");
        let out_dir = Path::new(out_path).join(item_name);
        create_dir_all(out_dir.clone()).map_err(|err| FSError {
            path: out_dir.clone(),
            err_msg: err.to_string(),
            access: AccessType::Write,
        })?;
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
