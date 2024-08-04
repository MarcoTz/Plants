use super::errors::{AccessType, Error, FSError};
use std::path;

pub struct FileDB {
    pub plants_dir: String,
    pub species_dir: String,
    logs_dir: String,
    graveyard_csv: String,
    growth_csv: String,
    activities_csv: String,
    pub date_format: String,
    //REMOVE later
    pub plants_out_dir: String,
    pub species_out_dir: String,
    pub graveyard_out: String,
    pub activities_out: String,
    pub growth_out: String,
}

fn get_path_from_buf(logs_dir: &str, file_name: &str) -> Result<String, Error> {
    let file_path = path::Path::new(logs_dir).join(file_name);
    match file_path.to_str() {
        None => Err(FSError {
            file_name: file_name.to_owned(),
            err_msg: "Could not find path".to_owned(),
            access: AccessType::Read,
        }
        .into()),
        Some(st) => Ok(st.to_owned()),
    }
}
impl FileDB {
    pub fn get_activities_filepath(&self) -> Result<String, Error> {
        get_path_from_buf(&self.logs_dir, &self.activities_csv)
    }

    pub fn get_graveyard_filepath(&self) -> Result<String, Error> {
        get_path_from_buf(&self.logs_dir, &self.graveyard_csv)
    }

    pub fn get_growth_filepath(&self) -> Result<String, Error> {
        get_path_from_buf(&self.logs_dir, &self.growth_csv)
    }
}

pub fn get_default() -> FileDB {
    FileDB {
        plants_dir: "data/Plants".to_owned(),
        species_dir: "data/PlantSpecies".to_owned(),
        logs_dir: "data/Logs".to_owned(),
        graveyard_csv: "Graveyard.csv".to_owned(),
        growth_csv: "Growth.csv".to_owned(),
        activities_csv: "Activities.csv".to_owned(),
        date_format: "%d.%m.%Y".to_owned(),
        plants_out_dir: "data_new/Plants".to_owned(),
        species_out_dir: "data_new/Species".to_owned(),
        graveyard_out: "data_new/Logs/Graveyard.csv".to_owned(),
        activities_out: "data_new/Logs/Activities.csv".to_owned(),
        growth_out: "data_new/Logs/Growth.csv".to_owned(),
    }
}
