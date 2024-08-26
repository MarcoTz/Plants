mod errors;
mod port_plants;
mod port_species;

use port_plants::port_plants;
use port_species::port_species;
use std::path::PathBuf;

const DATA_DIR_OLD: &str = "data_old";
const DATA_DIR_NEW: &str = "data";
const DATE_FORMAT: &str = "%d.%m.%Y";
const INTERACTIVE: bool = false;

pub fn main() {
    let in_dir = PathBuf::from(DATA_DIR_OLD);
    let out_dir = PathBuf::from(DATA_DIR_NEW);

    let plants_dir_in = in_dir.join("Plants");
    let plants_dir_out = out_dir.join("Plants");
    match port_plants(&plants_dir_in, DATE_FORMAT, &plants_dir_out) {
        Ok(()) => println!("Successfully ported plants"),
        Err(err) => println!("{err:?}"),
    };

    let species_dir_in = in_dir.join("PlantSpecies");
    let species_dir_out = out_dir.join("Species");
    match port_species(&species_dir_in, INTERACTIVE, &species_dir_out) {
        Ok(()) => println!("Successfully ported species"),
        Err(err) => println!("{err:?}"),
    }

    //1. Load all plants and save them again, ensuring all fields have the correct types
    //  species needs to be either species or string
    //2. Do the same for all species
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
