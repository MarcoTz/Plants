use super::constants::{ACTIVITIES_OUT, GRAVEYARD_OUT, GROWTH_OUT};
use super::errors::Error;
use csv::WriterBuilder;
use plants::graveyard::GraveyardPlant;
use plants::growth_item::GrowthItem;
use plants::log_item::LogItem;
use serde::Serialize;

fn write_csv<T: Serialize + std::fmt::Debug>(items: Vec<T>, file_path: &str) -> Result<(), Error> {
    println!("Writing {file_path}");
    let mut writer = WriterBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_path(file_path)
        .map_err(|err| Error::OtherErr(err.to_string()))?;
    for item in items.iter() {
        writer
            .serialize(item)
            .map_err(|err| Error::OtherErr(err.to_string()))?;
    }
    writer.flush()?;
    Ok(())
}

pub fn write_activities(activities: Vec<LogItem>) -> Result<(), Error> {
    write_csv(activities, ACTIVITIES_OUT)
}
pub fn write_growth(growth: Vec<GrowthItem>) -> Result<(), Error> {
    write_csv(growth, GROWTH_OUT)
}
pub fn write_graveyard(graveyard: Vec<GraveyardPlant>) -> Result<(), Error> {
    write_csv(graveyard, GRAVEYARD_OUT)
}
