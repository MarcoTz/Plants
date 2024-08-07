use database::file_backend::file_db;
use render_html::{
    renderer::{PageURLs, Renderer},
    write_html::write_html,
};

fn main() {
    let db_man = file_db::FileDB::get_default();
    let renderer = Renderer {
        database_manager: db_man,
        urls: PageURLs::get_default(),
        date_format: "%d.%m.%Y".to_owned(),
    };
    let index_str = renderer.render_index();
    let index_url = "html_out/".to_owned() + &renderer.urls.index_url;
    println!("{index_url}");
    match index_str {
        Ok(s) => match write_html(s, &index_url) {
            Ok(_) => println!("Successfully wrote index"),
            Err(err) => println!("{err:?}"),
        },
        Err(err) => println!("{err:?}"),
    }
}
