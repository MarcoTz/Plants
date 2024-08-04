use super::constants::{ACTIVITY_FILE, GRAVEYARD_FILE, GROWTH_FILE};
use super::csv_to_growth_item::GrowthCSV;
use super::csv_to_log_item::LogCSV;
use super::errors::Error;
use csv::ReaderBuilder;
use plants::graveyard::GraveyardPlant;
use plants::growth_item::GrowthItem;
use plants::log_item::LogItem;
use serde::de::DeserializeOwned;

fn load_csv<T: DeserializeOwned>(file_path: &str) -> Result<Vec<T>, Error> {
    let mut csv_rows = vec![];
    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_path(file_path)
        .map_err(|err| err.to_string())?;
    for csv_line in csv_reader.records() {
        let csv_record = csv_line.map_err(|err| err.to_string())?;
        let csv_item: T = csv_record
            .deserialize(None)
            .map_err(|err| err.to_string())?;
        csv_rows.push(csv_item);
    }
    Ok(csv_rows)
}

pub fn load_graveyard() -> Result<Vec<GraveyardPlant>, Error> {
    let mut graveyard = load_csv(GRAVEYARD_FILE)?;
    graveyard.sort();
    Ok(graveyard)
}

pub fn load_activities() -> Result<Vec<LogItem>, Error> {
    let activities_csv: Vec<LogCSV> = load_csv(ACTIVITY_FILE)?;
    let mut activities_conv = activities_csv
        .iter()
        .cloned()
        .flat_map(|x| <LogCSV as Into<Vec<LogItem>>>::into(x))
        .collect::<Vec<LogItem>>();
    activities_conv.sort();
    Ok(activities_conv)
}

pub fn load_growth() -> Result<Vec<GrowthItem>, Error> {
    let growth_csv: Vec<GrowthCSV> = load_csv(GROWTH_FILE)?;
    let mut growth_conv = growth_csv
        .iter()
        .cloned()
        .map(|x| <GrowthCSV as Into<GrowthItem>>::into(x))
        .collect::<Vec<GrowthItem>>();
    growth_conv.sort();
    Ok(growth_conv)
}
