use database::file_backend::file_db;
use render_html::renderer::{PageURLs, Renderer};

fn main() {
    let db_man = file_db::FileDB::get_default();
    let renderer = Renderer {
        database_manager: db_man,
        urls: PageURLs::get_default(),
        date_format: "%d.%m.%Y".to_owned(),
    };
    let index_str = renderer.render_index();
    match index_str {
        Ok(s) => println!("{s}"),
        Err(err) => println!("{err:?}"),
    }
}
