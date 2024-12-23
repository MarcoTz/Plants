#[macro_use]
extern crate rouille;

use database::{database_manager::DatabaseManager, sqlite_backend::SQLiteDB};
use render_html::renderer::Renderer;
use std::{fs::File, path::PathBuf, sync::Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = SQLiteDB::new(PathBuf::from("plants.db"))?;
    let renderer = Renderer::new(db, "%d.%m.%Y");
    let render_mutex = Mutex::new(renderer);
    rouille::start_server("0.0.0.0:8080", move |request| {
        router!(request, (GET) ["/"] =>{
                let content = render_mutex.lock()
                    .expect("Could not lock database")
                    .render_index().expect("Could not render index");
                rouille::Response::html(content)
            },
            (GET) ["/index.html"] => {

                let content = render_mutex.lock()
                    .expect("Could not lock database").
                    render_index().expect("Could not render index");
                rouille::Response::html(content)
            },
            (GET) ["/plant_overview.html"] => {
                let content = render_mutex.lock()
                    .expect("Could not lock database")
                    .render_plant_overview().expect("Could not render plant overview");
                rouille::Response::html(content)
            },
            (GET) ["/species_overview.html"] => {
                let content = render_mutex.lock()
                    .expect("Could not lock database")
                    .render_species_overview().expect("Could not render species overview");
                rouille::Response::html(content)
            },
            (GET) ["/gallery.html"] =>{
                let content = render_mutex.lock()
                    .expect("Could not lock database")
                    .render_gallery().expect("Could not render galley");
                rouille::Response::html(content)
            },
            (GET) ["/activities.html"] =>{
                let content = render_mutex.lock()
                    .expect("Could not lock database")
                    .render_activities().expect("Could not render galley");
                rouille::Response::html(content)
            },
            (GET) ["/graveyard.html"] =>{
                let content = render_mutex.lock()
                    .expect("Could not lock database")
                    .render_graveyard().expect("Could not render graveyard");
                rouille::Response::html(content)
            },
            (GET) ["/plants/{name}", name:String] => {
                let name = name.replace(".html","");
                let mut renderer = render_mutex.lock().expect("Could not lock database");
                let plant_name = renderer.database_manager.find_plant_name(name).expect("Could not find plant");
                let content =renderer
                    .render_plant_details(plant_name.clone())
                    .expect(&format!("Could not render details for plant {plant_name}"));
                rouille::Response::html(content.page_html)
            },
            (GET) ["/species/{name}", name:String] => {
                let name = name.replace(".html","");
                let mut renderer = render_mutex.lock().expect("Could not lock database");
                let species_name = renderer.database_manager.find_species_name(name).expect("Could not find species");
                let content =renderer
                    .render_species_details(species_name.clone())
                    .expect(&format!("Could not render details for species {species_name}"));
                rouille::Response::html(content.page_html)
            },
            (GET) ["/img/{plant}/{img}", plant:String,img:String] => {
                let ext = img.split(".").last().unwrap_or("jpg");
                let file = File::open(format!("html_out/img/{plant}/{img}")).expect("Could not find image");
                rouille::Response::from_file(format!("image/{ext}"),file)
            },
            (GET) ["/js/{file}", file:String] => {
                let file = File::open(format!("html_out/js/{file}")).expect("Could not find image");
                rouille::Response::from_file(format!("text/javascript"),file)
            },
            _ => {
                if request.url().contains("//"){
                    return rouille::Response::redirect_303(request.url().replace("//","/"));
                }
                println!("tried to serve {}",request.url());
                let mut response =rouille::Response::text(format!("Could not find page {}",request.url()));
                response.status_code=404;
                response
            }
        )
    })
}
