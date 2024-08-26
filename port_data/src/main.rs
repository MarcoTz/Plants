mod errors;
mod port;

use database::file_backend::load_json::load_dir;
use plants::{growth_item::GrowthItem, plant::PlantInfo, species::Species};
use port::{growth::GrowthCSV, plants::PlantJSON, species::SpeciesJSON, Port};
use std::path::PathBuf;

const DATA_DIR_OLD: &str = "data_old";
const DATA_DIR_NEW: &str = "data";
const PLANTS_DIR: &str = "Plants";
const SPECIES_DIR_IN: &str = "PlantSpecies";
const SPECIES_DIR_OUT: &str = "Species";
const LOGS_DIR: &str = "Logs";
const GROWTH_CSV: &str = "Growth.csv";
const DATE_FORMAT: &str = "%d.%m.%Y";
const INTERACTIVE: bool = false;

pub fn main() {
    let in_dir = PathBuf::from(DATA_DIR_OLD);
    let out_dir = PathBuf::from(DATA_DIR_NEW);
    let log_path_in = in_dir.join(LOGS_DIR);
    let log_path_out = out_dir.join(LOGS_DIR);

    let plants_dir_in = in_dir.join(PLANTS_DIR);
    let plants_dir_out = out_dir.join(PLANTS_DIR);
    match <Vec<PlantJSON> as Port<Vec<PlantInfo>>>::port(
        &plants_dir_in,
        &DATE_FORMAT.to_string(),
        &plants_dir_out,
    ) {
        Ok(()) => println!("Successfully ported plants"),
        Err(err) => println!("{err:?}"),
    };

    let species_dir_in = in_dir.join(SPECIES_DIR_IN);
    let species_dir_out = out_dir.join(SPECIES_DIR_OUT);
    match <Vec<SpeciesJSON> as Port<Vec<Species>>>::port(
        &species_dir_in,
        &INTERACTIVE,
        &species_dir_out,
    ) {
        Ok(()) => println!("Successfully ported species"),
        Err(err) => println!("{err:?}"),
    }

    let growth_file_in = log_path_in.join(GROWTH_CSV);
    let growth_file_out = log_path_out.join(GROWTH_CSV);
    let plant_jsons: Vec<PlantJSON> = load_dir(&plants_dir_in).unwrap_or(vec![]);
    match <Vec<GrowthCSV> as Port<Vec<GrowthItem>>>::port(
        &growth_file_in,
        &plant_jsons,
        &growth_file_out,
    ) {
        Ok(()) => println!("Successfully ported growth"),
        Err(err) => println!("{err:?}"),
    };

    //3. Add health to growth logs (currnent health for last log
    //4. All plant images need to be in a directory with the plants
    //  directory structure should be
    //      | - plants
    //          | - plant_name
    //              | - image1.jpg
    //              | - image2.jpg
    //              | - ...
    //              | - data.json
    //          | - plant_name
    //              | - ...
    //
    //
    /*let last_health = plant_json.plant_health.parse::<i32>()?;
    let mut last_growth =
        plant_growth
            .pop()
            .ok_or(Error::PlantError(plants::errors::Error::GrowthError(
                plant_info.name.clone(),
            )))?;
    last_growth.health = last_health;
    plant_growth.push(last_growth);*/
}
