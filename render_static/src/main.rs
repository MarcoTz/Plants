use database::errors::DBError;
use database::json_db::{load_plants, load_species};
use plants::plant::Plant;
use plants::species::Species;

fn load_json() -> Result<(Vec<Plant>, Vec<Species>), DBError> {
    let plant_list = load_plants()?;
    let species_list = load_species()?;
    Ok((plant_list, species_list))
}
fn main() {
    let plants_species = load_json();
    match plants_species {
        Err(err) => println!("Error: {:?}", err),
        Ok(_) => println!("Successfully loaded json"),
    }
}
