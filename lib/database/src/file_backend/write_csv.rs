use super::errors::{AccessType, CSVError, Error, FSError, SerializeError};
use csv::WriterBuilder;
use plants::{graveyard::GraveyardPlant, growth_item::GrowthItem, log_item::LogItem};
use serde::Serialize;
use std::path::PathBuf;

pub fn write_csv<T: Serialize + std::fmt::Debug>(
    items: Vec<T>,
    file_path: &PathBuf,
) -> Result<(), Error> {
    log::info!("Writing CSV {:?}", file_path);
    let mut writer = WriterBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_path(file_path)
        .map_err(|err| CSVError {
            path: file_path.clone(),
            err_msg: err.to_string(),
        })?;
    for item in items.iter() {
        writer.serialize(item).map_err(|err| SerializeError {
            path: file_path.clone(),
            err_msg: err.to_string(),
            access: AccessType::Write,
        })?;
    }
    writer.flush().map_err(|err| FSError {
        path: file_path.clone(),
        err_msg: err.to_string(),
        access: AccessType::Write,
    })?;
    Ok(())
}

pub fn write_activities(activities: Vec<LogItem>, activities_out: &PathBuf) -> Result<(), Error> {
    write_csv(activities, activities_out)
}
pub fn write_growth(growth: Vec<GrowthItem>, growth_out: &PathBuf) -> Result<(), Error> {
    write_csv(growth, growth_out)
}
pub fn write_graveyard(
    graveyard: Vec<GraveyardPlant>,
    graveyard_out: &PathBuf,
) -> Result<(), Error> {
    write_csv(graveyard, graveyard_out)
}
