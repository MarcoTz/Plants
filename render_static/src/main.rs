use database::file_backend::errors::Error;
use database::file_backend::load_csv::{load_activities, load_graveyard, load_growth};
//use database::file_backend::load_json::{load_plants, load_species};
//use database::file_backend::write_json::{write_plants, write_species};
use database::file_backend::write_csv::{write_activities, write_graveyard, write_growth};
use plants::graveyard::GraveyardPlant;
use plants::growth_item::GrowthItem;
use plants::log_item::LogItem;
use plants::plant::Plant;
use plants::species::Species;

fn load() -> Result<(Vec<LogItem>, Vec<GrowthItem>, Vec<GraveyardPlant>), Error> {
    //-> Result<(Vec<Plant>, Vec<Species>), Error> {
    //let plants = load_plants()?;
    //let species = load_species()?;
    //Ok((plants, species))
    let logs = load_activities()?;
    let growth = load_growth()?;
    let graveyard = load_graveyard()?;
    Ok((logs, growth, graveyard))
}
fn save(
    logs: Vec<LogItem>,
    growth: Vec<GrowthItem>,
    graveyard: Vec<GraveyardPlant>,
) -> Result<(), Error> {
    //(plants: Vec<Plant>, species: Vec<Species>) -> Result<(), Error> {
    //    write_plants(plants)?;
    //    write_species(species)?;

    write_activities(logs)?;
    write_growth(growth)?;
    write_graveyard(graveyard)?;
    Ok(())
}
fn main() {
    match load() {
        Err(err) => println!("{err:?}"),
        //Ok((plants, species)) => match save(plants, species) {
        Ok((logs, growth, graveyard)) => match save(logs, growth, graveyard) {
            Ok(()) => return,
            Err(err) => println!("{err:?}"),
        },
    }
}
