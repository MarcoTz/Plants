use super::errors::{CSVError, Error, SerializeError};
use csv::ReaderBuilder;
use plants::{
    graveyard::GraveyardPlant, growth_item::GrowthItem, location::Location, log_item::LogItem,
};
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
        })?;
        let csv_item: T = csv_record.deserialize(None).map_err(|err| SerializeError {
            path: file_path.clone(),
            err_msg: err.to_string(),
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
    let mut activities_csv: Vec<LogItem> = load_csv(activity_file)?;
    activities_csv.sort();
    Ok(activities_csv)
}

pub fn load_growth(growth_file: &PathBuf) -> Result<Vec<GrowthItem>, Error> {
    let mut growth_csv: Vec<GrowthItem> = load_csv(growth_file)?;
    growth_csv.sort();
    Ok(growth_csv)
}

pub fn load_locations(location_file: &PathBuf) -> Result<Vec<Location>, Error> {
    let locations: Vec<Location> = load_csv(location_file)?;
    Ok(locations)
}

#[cfg(test)]
mod load_csv_tests {
    use super::{load_activities, load_csv, load_graveyard, load_growth, load_locations};
    use crate::file_backend::test_common::dummy_date;
    use chrono::NaiveDate;
    use plants::{
        graveyard::GraveyardPlant, growth_item::GrowthItem, location::Location, log_item::LogItem,
        serialize::date_serializer,
    };
    use serde::Deserialize;
    use std::path::PathBuf;

    const CSV_DUMMY: &str = "../../testing/dummy.csv";
    const CSV_DUMMY_BAD_LINES: &str = "../../testing/dummy_badlines.csv";
    const CSV_DUMMY_DESERIALIZE: &str = "../../testing/dummy_deserialize.csv";
    const GRAVEYARD_DUMMY: &str = "../../testing/Logs/Graveyard.csv";
    const ACTIVITIES_DUMMY: &str = "../../testing/Logs/Activities.csv";
    const GROWTH_DUMMY: &str = "../../testing/Logs/Growth.csv";
    const LOCATIONS_DUMMY: &str = "../../testing/Locations.csv";

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct CSVDummy {
        field1: String,
        field2: i64,
        #[serde(with = "date_serializer")]
        field3: NaiveDate,
        field4: Option<String>,
    }

    #[test]
    fn test_csv() {
        let result: Vec<CSVDummy> = load_csv(&PathBuf::from(&CSV_DUMMY)).unwrap();
        let expected = vec![
            CSVDummy {
                field1: "value1".to_owned(),
                field2: 1,
                field3: dummy_date(),
                field4: Some("value2".to_owned()),
            },
            CSVDummy {
                field1: "value3".to_owned(),
                field2: 2,
                field3: dummy_date(),
                field4: None,
            },
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_csv_bad_line() {
        let result = load_csv::<Vec<CSVDummy>>(&PathBuf::from(&CSV_DUMMY_BAD_LINES));
        assert!(result.is_err())
    }

    #[test]
    fn test_csv_deserialize() {
        let result = load_csv::<Vec<CSVDummy>>(&PathBuf::from(&CSV_DUMMY_DESERIALIZE));
        assert!(result.is_err())
    }

    #[test]
    fn test_load_graveyad() {
        let result = load_graveyard(&PathBuf::from(&GRAVEYARD_DUMMY)).unwrap();
        let expected = vec![
            GraveyardPlant {
                name: "Dummy1".to_owned(),
                species: "test species".to_owned(),
                planted: dummy_date(),
                died: NaiveDate::parse_from_str("02.01.1970", "%d.%m.%Y").unwrap(),
                reason: "testing".to_owned(),
            },
            GraveyardPlant {
                name: "Dummy2".to_owned(),
                species: "testing species".to_owned(),
                planted: dummy_date(),
                died: NaiveDate::parse_from_str("02.01.1970", "%d.%m.%Y").unwrap(),
                reason: "testing".to_owned(),
            },
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_graveyard_fail() {
        let result = load_graveyard(&PathBuf::from("../../testing/notafile"));
        assert!(result.is_err())
    }

    #[test]
    fn test_activities() {
        let result = load_activities(&PathBuf::from(&ACTIVITIES_DUMMY)).unwrap();
        let expected = vec![LogItem {
            activity: "Watering".to_owned(),
            date: dummy_date(),
            plant: "Dummy1".to_owned(),
            note: None,
        }];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_activities_fail() {
        let result = load_graveyard(&PathBuf::from("../../testing/notafile"));
        assert!(result.is_err())
    }

    #[test]
    fn test_growth() {
        let result = load_growth(&PathBuf::from(&GROWTH_DUMMY)).unwrap();
        let expected = vec![GrowthItem {
            plant: "Dummy1".to_owned(),
            date: dummy_date(),
            height_cm: 10.0,
            width_cm: 10.0,
            note: None,
            health: 3,
        }];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_growth_file() {
        let result = load_growth(&PathBuf::from("../../testing/notafile"));
        assert!(result.is_err())
    }

    #[test]
    fn test_locations() {
        let result = load_locations(&PathBuf::from(&LOCATIONS_DUMMY)).unwrap();
        let expected = vec![
            Location {
                name: "test outside".to_owned(),
                outside: true,
            },
            Location {
                name: "test inside".to_owned(),
                outside: false,
            },
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_locations_fail() {
        let result = load_locations(&PathBuf::from("../../testing/notafile"));
        assert!(result.is_err())
    }
}
