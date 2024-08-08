use database::file_backend::file_db;
use render_html::{
    renderer::{PageURLs, Renderer},
    write_html::write_all,
};

fn main() {
    let db_man = file_db::FileDB::get_default();
    let mut renderer = Renderer {
        database_manager: db_man,
        urls: PageURLs::get_default(),
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
