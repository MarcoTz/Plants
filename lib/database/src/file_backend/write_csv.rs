use super::errors::Error;
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
        .map_err(|err| Error::OtherErr(err.to_string()))?;
    for item in items.iter() {
        writer
            .serialize(item)
            .map_err(|err| Error::OtherErr(err.to_string()))?;
    }
    writer.flush()?;
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
