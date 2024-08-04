use database::file_backend::errors::Error;
use database::file_backend::file_db;
use database::file_backend::load_csv::{load_activities, load_graveyard, load_growth};
use database::file_backend::load_json::{load_plants, load_species};
use database::file_backend::write_csv::{write_activities, write_graveyard, write_growth};
use database::file_backend::write_json::{write_plants, write_species};
use plants::graveyard::GraveyardPlant;
use plants::growth_item::GrowthItem;
use plants::log_item::LogItem;
use plants::plant::Plant;
use plants::species::Species;

fn load(
    db_man: &file_db::FileDB,
) -> Result<
    (
        Vec<Plant>,
        Vec<Species>,
        Vec<LogItem>,
        Vec<GrowthItem>,
        Vec<GraveyardPlant>,
    ),
    Error,
> {
    let plants = load_plants(&db_man.plants_dir)?;
    let species = load_species(&db_man.species_dir)?;
    let logs = load_activities(&db_man.get_activities_filepath()?)?;
    let growth = load_growth(&db_man.get_growth_filepath()?)?;
    let graveyard = load_graveyard(&db_man.get_graveyard_filepath()?)?;
    Ok((plants, species, logs, growth, graveyard))
}
fn save(
    db_man: &file_db::FileDB,
    plants: Vec<Plant>,
    species: Vec<Species>,
    logs: Vec<LogItem>,
    growth: Vec<GrowthItem>,
    graveyard: Vec<GraveyardPlant>,
) -> Result<(), Error> {
    write_plants(plants, &db_man.plants_out_dir)?;
    write_species(species, &db_man.species_out_dir)?;

    write_activities(logs, &db_man.activities_out)?;
    write_growth(growth, &db_man.growth_out)?;
    write_graveyard(graveyard, &db_man.graveyard_out)?;
    Ok(())
}
fn main() {
    let db_man = file_db::get_default();
    match load(&db_man) {
        Err(err) => println!("{err:?}"),
        //Ok((plants, species)) => match save(plants, species) {
        Ok((plants, species, logs, growth, graveyard)) => {
            match save(&db_man, plants, species, logs, growth, graveyard) {
                Ok(()) => return,
                Err(err) => println!("{err:?}"),
            }
        }
    }
}
