use super::errors::{AccessType, CSVError, Error, FSError, SerializeError};
use csv::WriterBuilder;
use plants::graveyard::GraveyardPlant;
use plants::growth_item::GrowthItem;
use plants::log_item::LogItem;
use serde::Serialize;

fn write_csv<T: Serialize + std::fmt::Debug>(items: Vec<T>, file_path: &str) -> Result<(), Error> {
    let mut writer = WriterBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_path(file_path)
        .map_err(|err| {
            <CSVError as Into<Error>>::into(CSVError {
                csv_file: file_path.to_owned(),
                err_msg: err.to_string(),
            })
        })?;
    for item in items.iter() {
        writer.serialize(item).map_err(|err| {
            <SerializeError as Into<Error>>::into(SerializeError {
                out_path: file_path.to_owned(),
                err_msg: err.to_string(),
                access: AccessType::Write,
            })
        })?;
    }
    writer.flush().map_err(|err| {
        <FSError as Into<Error>>::into(FSError {
            file_name: file_path.to_owned(),
            err_msg: err.to_string(),
            access: AccessType::Write,
        })
    })?;
    Ok(())
}

pub fn write_activities(activities: Vec<LogItem>, activities_out: &str) -> Result<(), Error> {
    write_csv(activities, activities_out)
}
pub fn write_growth(growth: Vec<GrowthItem>, growth_out: &str) -> Result<(), Error> {
    write_csv(growth, growth_out)
}
pub fn write_graveyard(graveyard: Vec<GraveyardPlant>, graveyard_out: &str) -> Result<(), Error> {
    write_csv(graveyard, graveyard_out)
}
