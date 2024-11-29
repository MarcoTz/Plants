use database::{database_manager::DatabaseManager, sqlite_backend::SQLiteDB};
use std::path::PathBuf;

use chrono::NaiveDate;
use plants::growth_item::GrowthItem;

fn main() -> Result<(), String> {
    let mut db_man = SQLiteDB::new(PathBuf::from("plants.db")).map_err(|err| err.to_string())?;
    let res = db_man
        .write_growths(vec![
            GrowthItem {
                date: NaiveDate::parse_from_str("01.01.2024", "%d.%m.%Y").unwrap(),
                plant: "testing".to_owned(),
                height_cm: 5.5,
                width_cm: 6.0,
                note: Some("testing".to_owned()),
                health: 5,
            },
            GrowthItem {
                date: NaiveDate::parse_from_str("02.01.2024", "%d.%m.%Y").unwrap(),
                plant: "othertest".to_owned(),
                height_cm: 4.0,
                width_cm: 3.0,
                note: None,
                health: 4,
            },
        ])
        .map_err(|err| err.to_string())?;
    println!("{:?}", res);
    Ok(())
}
