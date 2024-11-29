use database::{database_manager::DatabaseManager, sqlite_backend::SQLiteDB};
use std::path::PathBuf;

use chrono::NaiveDate;
use plants::log_item::LogItem;

fn main() -> Result<(), String> {
    let mut db_man = SQLiteDB::new(PathBuf::from("plants.db")).map_err(|err| err.to_string())?;
    let res = db_man.get_all_plants().map_err(|err| err.to_string())?;

    /*    let res = db_man
    .write_logs(vec![
        LogItem {
            activity: "test".to_owned(),
            date: NaiveDate::parse_from_str("01.01.2024", "%d.%m.%Y").unwrap(),
            plant: "testing".to_owned(),
            note: Some("testing".to_owned()),
        },
        LogItem {
            activity: "testing".to_owned(),
            date: NaiveDate::parse_from_str("02.01.2024", "%d.%m.%Y").unwrap(),
            plant: "othertest".to_owned(),
            note: None,
        },
    ])
    .map_err(|err| err.to_string())?;*/
    println!("{:?}", res);
    Ok(())
}
