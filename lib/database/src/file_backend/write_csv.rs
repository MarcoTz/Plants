use super::errors::{Error, SerializeError};
use csv::WriterBuilder;
use plants::{
    graveyard::GraveyardPlant, growth_item::GrowthItem, location::Location, log_item::LogItem,
};
use serde::Serialize;
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

pub fn write_csv<T: Serialize>(
    items: Vec<T>,
    file_path: &PathBuf,
    append: bool,
) -> Result<(), Error> {
    log::info!("Writing CSV {:?}", file_path);
    let mut headers = !append;
    if !file_path.exists() {
        File::create(file_path)?;
        headers = true;
    }
    let file = OpenOptions::new()
        .write(true)
        .append(append)
        .open(file_path)?;
    let mut writer = WriterBuilder::new()
        .delimiter(b';')
        .has_headers(headers)
        .flexible(true)
        .from_writer(file);
    for item in items.iter() {
        writer.serialize(item).map_err(|err| SerializeError {
            path: file_path.clone(),
            err_msg: err.to_string(),
        })?;
    }
    writer.flush()?;
    Ok(())
}

pub fn write_activities(
    activities: Vec<LogItem>,
    activities_out: &PathBuf,
    append: bool,
) -> Result<(), Error> {
    write_csv(activities, activities_out, append)
}
pub fn write_growth(
    growth: Vec<GrowthItem>,
    growth_out: &PathBuf,
    append: bool,
) -> Result<(), Error> {
    write_csv(growth, growth_out, append)
}
pub fn write_graveyard(
    graveyard: Vec<GraveyardPlant>,
    graveyard_out: &PathBuf,
    append: bool,
) -> Result<(), Error> {
    write_csv(graveyard, graveyard_out, append)
}

pub fn add_location(location: Location, location_out: &PathBuf) -> Result<(), Error> {
    write_csv(vec![location], location_out, true)
}

#[cfg(test)]
mod write_csv_tests {
    use super::{write_activities, write_csv, write_graveyard, write_growth};
    use crate::file_backend::{
        load_csv::{load_activities, load_csv, load_graveyard, load_growth},
        test_common::{
            dummy_activity, dummy_date, dummy_graveyard1, dummy_graveyard2, dummy_growth1,
            dummy_growth2, DummyCSV, ACTIVITIES_CSV_DUMMY_OUT, CSV_DUMMY_OUT,
            GRAVEYARD_CSV_DUMMY_OUT, GROWTH_CSV_DUMMY_OUT,
        },
    };
    use std::path::PathBuf;

    fn csv_dummy() -> DummyCSV {
        DummyCSV {
            key1: "test".to_owned(),
            key2: 1,
            key3: dummy_date(),
            key4: 1.0,
        }
    }

    #[test]
    fn write_dummy_csv() {
        let values = vec![csv_dummy(), csv_dummy()];
        let csv_file = PathBuf::from(CSV_DUMMY_OUT);
        write_csv(values, &csv_file, true).unwrap();
        let result: Vec<DummyCSV> = load_csv(&csv_file).unwrap();
        let expected = vec![csv_dummy(), csv_dummy()];
        assert_eq!(result, expected);
        std::fs::remove_file(csv_file.clone()).unwrap();
        assert!(!csv_file.exists())
    }

    #[test]
    fn write_dummy_activities() {
        let values = vec![dummy_activity(), dummy_activity()];
        let csv_file = PathBuf::from(ACTIVITIES_CSV_DUMMY_OUT);
        write_activities(values, &csv_file, true).unwrap();
        let result = load_activities(&csv_file).unwrap();
        let expected = vec![dummy_activity(), dummy_activity()];
        assert_eq!(result, expected);
        std::fs::remove_file(csv_file.clone()).unwrap();
        assert!(!csv_file.exists());
    }

    #[test]
    fn write_dummy_growth() {
        let values = vec![dummy_growth1(), dummy_growth2()];
        let csv_file = PathBuf::from(GROWTH_CSV_DUMMY_OUT);
        write_growth(values, &csv_file, true).unwrap();
        let result = load_growth(&csv_file).unwrap();
        let expected = vec![dummy_growth1(), dummy_growth2()];
        assert_eq!(result, expected);
        std::fs::remove_file(csv_file.clone()).unwrap();
        assert!(!csv_file.exists())
    }

    #[test]
    fn write_dummy_graveyard() {
        let values = vec![dummy_graveyard1(), dummy_graveyard2()];
        let csv_file = PathBuf::from(GRAVEYARD_CSV_DUMMY_OUT);
        write_graveyard(values, &csv_file, true).unwrap();
        let result = load_graveyard(&csv_file).unwrap();
        let expected = vec![dummy_graveyard1(), dummy_graveyard2()];
        assert_eq!(result, expected);
        std::fs::remove_file(csv_file.clone()).unwrap();
        assert!(!csv_file.exists())
    }
}
