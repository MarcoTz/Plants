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