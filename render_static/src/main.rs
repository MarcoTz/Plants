use database::sqlite_backend::SQLiteDB;
use log::Level;
use logger::{file_logger::FileLogger, init::init_logger};
use render_html::{renderer::Renderer, write_html::write_all};
use std::path::PathBuf;

static LOGGER: FileLogger = FileLogger {
    level: Level::Info,
    file_path: "build.log",
};

fn main() -> Result<(), String> {
    init_logger(&LOGGER)?;

    let db_man = SQLiteDB::new(PathBuf::from("plants.db")).map_err(|err| err.to_string())?;

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
