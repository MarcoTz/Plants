use super::{
    csv_to_log_item::LogCSV,
    errors::{AccessType, CSVError, Error, SerializeError},
};
use csv::ReaderBuilder;
use plants::{graveyard::GraveyardPlant, growth_item::GrowthItem, log_item::LogItem};
use serde::de::DeserializeOwned;
use std::path::PathBuf;

pub fn load_csv<T: DeserializeOwned>(file_path: &PathBuf) -> Result<Vec<T>, Error> {
    log::info!("Loading CSV {:?}", file_path);
    let mut csv_rows = vec![];
    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_path(file_path)
        .map_err(|err| CSVError {
            path: file_path.clone(),
            err_msg: err.to_string(),
        })?;

    for csv_line in csv_reader.records() {
        let csv_record = csv_line.map_err(|err| SerializeError {
            path: file_path.clone(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })?;
        let csv_item: T = csv_record.deserialize(None).map_err(|err| SerializeError {
            path: file_path.clone(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })?;
        csv_rows.push(csv_item);
    }
    Ok(csv_rows)
}

pub fn load_graveyard(graveyard_file: &PathBuf) -> Result<Vec<GraveyardPlant>, Error> {
    let mut graveyard = load_csv(graveyard_file)?;
    graveyard.sort();
    Ok(graveyard)
}

pub fn load_activities(activity_file: &PathBuf) -> Result<Vec<LogItem>, Error> {
    let activities_csv: Vec<LogCSV> = load_csv(activity_file)?;
    let mut activities_conv = activities_csv
        .iter()
        .cloned()
        .flat_map(<LogCSV as Into<Vec<LogItem>>>::into)
        .collect::<Vec<LogItem>>();
    activities_conv.sort();
    Ok(activities_conv)
}

pub fn load_growth(growth_file: &PathBuf) -> Result<Vec<GrowthItem>, Error> {
    let mut growth_csv: Vec<GrowthItem> = load_csv(growth_file)?;
    growth_csv.sort();
    Ok(growth_csv)
}
