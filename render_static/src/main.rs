use database::file_backend::errors::Error;
use database::file_backend::load_csv::{load_activities, load_graveyard, load_growth};
use database::file_backend::load_json::{load_plants, load_species};
use plants::plant::Plant;
use plants::species::Species;

fn load_res<T>(load_res: Result<T, Error>) {
    match load_res {
        Ok(_) => return,
        Err(err) => println!("{err:?}"),
    }
}
fn main() {
    load_res(load_activities());
    load_res(load_graveyard());
    load_res(load_growth());
    /*    let plants_species = load_json();
    match plants_species {
        Err(err) => println!("Error: {:?}", err),
        Ok(_) => println!("Successfully loaded json"),
    }*/
}
