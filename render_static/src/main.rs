use database::{database_manager::DatabaseManager, sqlite_backend::SQLiteDB};
use log::Level;
use logger::{file_logger::FileLogger, init::init_logger};
use render_html::{renderer::Renderer, write_html::write_all};
use std::path::PathBuf;

use chrono::NaiveDate;
use plants::growth_item::GrowthItem;

static LOGGER: FileLogger = FileLogger {
    level: Level::Info,
    file_path: "build.log",
};

fn main() -> Result<(), String> {
    init_logger(&LOGGER)?;

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

    std::process::exit(0);
    let mut renderer = Renderer {
        database_manager: db_man,
        date_format: "%d.%m.%Y".to_owned(),
    };

    log::info!("Rendering Pages");
    let pages = renderer.render_all().map_err(|err| err.to_string())?;
    log::info!("Wrote page htmls");
    write_all(pages, "html_out", "plants", "species").map_err(|err| err.to_string())?;
    log::info!("Successfully rendered pages");
    Ok(())
}
