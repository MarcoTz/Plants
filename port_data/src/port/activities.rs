use super::Port;
use crate::errors::Error;
use chrono::NaiveDate;
use database::file_backend::{load_csv::load_csv, write_csv::write_csv};
use plants::{log_item::LogItem, serialize::date_serializer};
use serde::Deserialize;
use std::{fs::File, path::PathBuf};

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
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
        write_csv(new_items, activities_file, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod activities_tests {
    use super::{LogCSV, LogItem, Port};
    use crate::port::test_common::{
        example_date1, example_date2, BASE_DIR, LOGS_FILE_IN, LOGS_FILE_OUT,
    };
    use database::file_backend::load_csv::load_csv;
    use std::path::PathBuf;

    fn example_log_csv1() -> LogCSV {
        LogCSV {
            date: example_date1(),
            activity: "Watering".to_owned(),
            plants: "Plant1,Plant2".to_owned(),
            note: None,
        }
    }

    fn example_log1() -> LogItem {
        LogItem {
            date: example_date1(),
            activity: "Watering".to_owned(),
            plant: "Plant1".to_owned(),
            note: None,
        }
    }

    fn example_log2() -> LogItem {
        LogItem {
            date: example_date1(),
            activity: "Watering".to_owned(),
            plant: "Plant2".to_owned(),
            note: None,
        }
    }

    fn example_log_csv2() -> LogCSV {
        LogCSV {
            date: example_date2(),
            activity: "Fertilizing".to_owned(),
            plants: "Plant2,Plant3".to_owned(),
            note: None,
        }
    }

    fn example_log3() -> LogItem {
        LogItem {
            date: example_date2(),
            activity: "Fertilizing".to_owned(),
            plant: "Plant2".to_owned(),
            note: None,
        }
    }

    fn example_log4() -> LogItem {
        LogItem {
            date: example_date2(),
            activity: "Fertilizing".to_owned(),
            plant: "Plant3".to_owned(),
            note: None,
        }
    }

    #[test]
    fn csv_to_log() {
        let result = <LogCSV as Into<Vec<LogItem>>>::into(LogCSV {
            date: example_date1(),
            activity: "Watering".to_owned(),
            plants: "Plant1,Plant2".to_owned(),
            note: None,
        });
        let expected = vec![
            LogItem {
                activity: "Watering".to_owned(),
                date: example_date1(),
                plant: "Plant1".to_owned(),
                note: None,
            },
            LogItem {
                activity: "Watering".to_owned(),
                date: example_date1(),
                plant: "Plant2".to_owned(),
                note: None,
            },
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn load_old() {
        let log_file = PathBuf::from(BASE_DIR).join(LOGS_FILE_IN);
        println!("{:?}", log_file);
        let result = <Vec<LogCSV> as Port<Vec<LogItem>>>::load_old(&log_file).unwrap();
        let expected = vec![example_log_csv1(), example_log_csv2()];
        assert_eq!(result, expected)
    }

    #[test]
    fn convert() {
        let result = vec![example_log_csv1(), example_log_csv2()]
            .convert(&())
            .unwrap();
        let expected = vec![
            example_log1(),
            example_log2(),
            example_log3(),
            example_log4(),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn save_new() {
        let log_file = PathBuf::from(BASE_DIR).join(LOGS_FILE_OUT);
        if log_file.exists() {
            std::fs::remove_file(log_file.clone()).unwrap();
        }
        assert!(!log_file.exists());

        <Vec<LogCSV> as Port<Vec<LogItem>>>::save_new(
            vec![
                example_log1(),
                example_log2(),
                example_log3(),
                example_log4(),
            ],
            &log_file,
        )
        .unwrap();

        assert!(log_file.exists());
        let result: Vec<LogItem> = load_csv(&log_file).unwrap();
        let expected = vec![
            example_log1(),
            example_log2(),
            example_log3(),
            example_log4(),
        ];
        assert_eq!(result, expected);

        std::fs::remove_file(log_file.clone()).unwrap();
        assert!(!log_file.exists())
    }
}
