use super::{
    errors::{AccessType, Error, FSError, SerializeError},
    json_to_plant::PlantJSON,
};
use plants::named::Named;
use plants::species::Species;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

pub fn write_json<T: Serialize>(item: T, out_filepath: &str) -> Result<(), Error> {
    let serialized = serde_json::to_string(&item).map_err(|err| {
        <SerializeError as Into<Error>>::into(SerializeError {
            out_path: out_filepath.to_owned(),
            err_msg: err.to_string(),
            access: AccessType::Write,
        })
    })?;
    let mut out_file = File::create(out_filepath).map_err(|err| {
        <FSError as Into<Error>>::into(FSError {
            file_name: out_filepath.to_owned(),
            err_msg: err.to_string(),
            access: AccessType::Write,
        })
    })?;
    out_file.write_all(serialized.as_bytes()).map_err(|err| {
        <FSError as Into<Error>>::into(FSError {
            file_name: out_filepath.to_owned(),
            err_msg: err.to_string(),
            access: AccessType::Write,
        })
    })?;
    Ok(())
}

pub fn write_vec<T: Serialize + Named>(items: Vec<T>, out_path_base: &str) -> Result<(), Error> {
    for item in items.iter() {
        let mut out_path = out_path_base.to_owned();
        out_path.push('/');
        out_path.push_str(&item.get_name());
        out_path.push_str(".json");
        write_json(item, &out_path)?;
    }
    Ok(())
}

pub fn write_plants(plants: Vec<PlantJSON>, plant_dir: &str) -> Result<(), Error> {
    write_vec(plants, plant_dir)
}

pub fn write_species(species: Vec<Species>, species_dir: &str) -> Result<(), Error> {
    write_vec(species, species_dir)
}
