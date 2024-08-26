mod errors;
mod port;

use database::file_backend::load_json::load_dir;
use log::Level;
use logger::{file_logger::FileLogger, init::init_logger};
use plants::{
    growth_item::GrowthItem,
    log_item::LogItem,
    plant::{PlantImage, PlantInfo},
    species::Species,
};
use port::{
    activities::LogCSV, growth::GrowthCSV, images::OldImage, plants::PlantJSON,
    species::SpeciesJSON, Port,
};
use std::{fs::create_dir, path::PathBuf, process::exit};

const DATA_DIR_OLD: &str = "data_old";
const DATA_DIR_NEW: &str = "data";
const PLANTS_DIR: &str = "Plants";
const SPECIES_DIR_IN: &str = "PlantSpecies";
const SPECIES_DIR_OUT: &str = "Species";
const LOGS_DIR: &str = "Logs";
const GROWTH_CSV: &str = "Growth.csv";
const ACTIVITIES_CSV: &str = "Activities.csv";
const DATE_FORMAT: &str = "%d.%m.%Y";
const INTERACTIVE: bool = false;

static LOGGER: FileLogger = FileLogger {
    level: Level::Info,
    file_path: "log.txt",
};

pub fn main() {
    let log_res = init_logger(&LOGGER);
    if log_res.is_err() {
        println!("{}", log_res.unwrap_err());
        std::process::exit(1);
    }

    let in_dir = PathBuf::from(DATA_DIR_OLD);
    let out_dir = PathBuf::from(DATA_DIR_NEW);
    if !out_dir.exists() {
        let create_res = create_dir(out_dir.clone());
        if create_res.is_err() {
            log::error!("Could not create data out dir");
            exit(1)
        }
    }
    let log_path_in = in_dir.join(LOGS_DIR);
    let log_path_out = out_dir.join(LOGS_DIR);
    if !log_path_out.exists() {
        let create_res = create_dir(log_path_out.clone());
        if create_res.is_err() {
            log::error!("Could not create logs out dir");
            exit(1)
        }
    }

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

    let activities_file_in = log_path_in.join(ACTIVITIES_CSV);
    let activities_file_out = log_path_out.join(ACTIVITIES_CSV);
    match <Vec<LogCSV> as Port<Vec<LogItem>>>::port(&activities_file_in, &(), &activities_file_out)
    {
        Ok(()) => println!("Successfully ported activities"),
        Err(err) => println!("{err:?}"),
    }

    let images_dir_in = in_dir.join("img");
    match <Vec<OldImage> as Port<Vec<PlantImage>>>::port(
        &images_dir_in,
        &(plants_dir_out, "%d%m%Y".to_owned()),
        &PathBuf::new(),
    ) {
        Ok(()) => println!("Successfully ported images"),
        Err(err) => println!("{err:?}"),
    }

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
}
