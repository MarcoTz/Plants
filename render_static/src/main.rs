use database::file_backend::FileDB;
use log::Level;
use logger::{file_logger::FileLogger, init::init_logger};
use render_html::{renderer::Renderer, write_html::write_all};

static LOGGER: FileLogger = FileLogger {
    level: Level::Warn,
    file_path: "log.txt",
};

fn main() {
    let log_res = init_logger(&LOGGER);
    if log_res.is_err() {
        println!("{}", log_res.unwrap_err());
        std::process::exit(1);
    }

    let db_man = FileDB::default();
    let mut renderer = Renderer {
        database_manager: db_man,
        date_format: "%d.%m.%Y".to_owned(),
    };
    let page_htmls = renderer.render_all();
    match page_htmls {
        Err(err) => println!("{err:?}"),
        Ok(htmls) => match write_all(htmls, "html_out/") {
            Err(err) => println!("{err:?}"),
            Ok(_) => println!("Successfully wrote html"),
        },
    }
}
