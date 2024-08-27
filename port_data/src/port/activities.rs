use super::Port;
use crate::errors::Error;
use chrono::NaiveDate;
use database::file_backend::{load_csv::load_csv, write_csv::write_csv};
use plants::{log_item::LogItem, serialize::date_serializer};
use serde::Deserialize;
use std::{fs::File, path::PathBuf};

#[derive(Deserialize, Clone)]
pub struct LogCSV {
    #[serde(with = "date_serializer")]
    date: NaiveDate,
    activity: String,
    plants: String,
    note: Option<String>,
}

impl From<LogCSV> for Vec<LogItem> {
    fn from(log_csv: LogCSV) -> Vec<LogItem> {
        let new_plants: Vec<String> = log_csv
            .plants
            .split(',')
            .map(|st| st.trim().to_owned())
            .collect();
        let mut items = vec![];
        for item in new_plants.iter() {
            let new_log = LogItem {
                date: log_csv.date,
                activity: log_csv.activity.clone(),
                plant: item.clone(),
                note: log_csv.note.clone(),
            };
            items.push(new_log);
        }
        items
    }
}

impl Port<Vec<LogItem>> for Vec<LogCSV> {
    type LoadArgs = PathBuf;
    type SaveArgs = PathBuf;
    type ConvertArgs = ();

    fn load_old(activities_file: &Self::LoadArgs) -> Result<Vec<LogCSV>, Error> {
        log::info!("Loading old acttivities");
        let csv_items: Vec<LogCSV> = load_csv(activities_file)?;
        Ok(csv_items)
    }
    fn convert(self, _: &Self::ConvertArgs) -> Result<Vec<LogItem>, Error> {
        log::info!("Converting activities");
        let mut new_items = vec![];
        for old_item in self.into_iter() {
            let new_item: Vec<LogItem> = old_item.into();
            new_items.extend(new_item);
        }
        Ok(new_items)
    }

    fn save_new(new_items: Vec<LogItem>, activities_file: &Self::SaveArgs) -> Result<(), Error> {
        log::info!("Saving new activities");
        if !activities_file.exists() {
            log::info!("Creating log file {activities_file:?}");
            File::create(activities_file)?;
        }
        write_csv(new_items, activities_file)?;
        Ok(())
    }
}
