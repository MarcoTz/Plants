use database::file_backend::errors::Error;
use database::file_backend::load_csv::{load_activities, load_graveyard, load_growth};
use database::file_backend::load_json::{load_plants, load_species};
use database::file_backend::write_json::{write_plants, write_species};
use plants::plant::Plant;
use plants::species::Species;

fn load() -> Result<(Vec<Plant>, Vec<Species>), Error> {
    let plants = load_plants()?;
    let species = load_species()?;
    Ok((plants, species))
}
fn save(plants: Vec<Plant>, species: Vec<Species>) -> Result<(), Error> {
    write_plants(plants)?;
    write_species(species)?;
    Ok(())
}
fn main() {
    match load() {
        Err(err) => println!("{err:?}"),
        Ok((plants, species)) => match save(plants, species) {
            Ok(()) => return,
            Err(err) => println!("{err:?}"),
        },
    }
}
