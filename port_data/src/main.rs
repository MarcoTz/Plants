mod dir_check;
mod errors;
mod port;

use dir_check::Directories;
use log::Level;
use logger::{file_logger::FileLogger, init::init_logger};
use plants::{
    growth_item::GrowthItem,
    location::Location,
    log_item::LogItem,
    plant::{PlantImage, PlantInfo},
    species::Species,
};
use port::{
    activities::LogCSV, growth::GrowthCSV, images::OldImage, plants::PlantJSON,
    species::SpeciesJSON, Port,
};
use std::{path::PathBuf, process::exit};

const LOCATIONS_CSV: &str = "Locations.csv";
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
        exit(1);
    }

    log::info!("Checking directories");
    let dirs = Directories::default();
    match dirs.ensure_exists() {
        Ok(()) => println!("Successfully checked all directories"),
        Err(err) => {
            println!("{err:?}");
            exit(1)
        }
    }

    log::info!("Porting Plants");
    match <Vec<PlantJSON> as Port<Vec<PlantInfo>>>::port(
        &dirs.plants_dir_in,
        &DATE_FORMAT.to_string(),
        &dirs.plants_dir_out,
    ) {
        Ok(()) => println!("Successfully ported plants"),
        Err(err) => {
            println!("{err:?}");
            exit(1)
        }
    };

    log::info!("Porting Species");
    match <Vec<SpeciesJSON> as Port<Vec<Species>>>::port(
        &dirs.species_dir_in,
        &INTERACTIVE,
        &dirs.species_dir_out,
    ) {
        Ok(()) => println!("Successfully ported species"),
        Err(err) => {
            println!("{err:?}");
            exit(1)
        }
    }

    log::info!("Porting Growth");
    let growth_file_in = dirs.logs_dir_in.join(GROWTH_CSV);
    let growth_file_out = dirs.logs_dir_out.join(GROWTH_CSV);
    let plant_jsons: Vec<PlantJSON> =
        match <Vec<PlantJSON> as Port<Vec<PlantInfo>>>::load_old(&dirs.plants_dir_in) {
            Ok(jsons) => jsons,
            Err(err) => {
                println!("{err:?}");
                exit(1);
            }
        };
    match <Vec<GrowthCSV> as Port<Vec<GrowthItem>>>::port(
        &growth_file_in,
        &plant_jsons,
        &growth_file_out,
    ) {
        Ok(()) => println!("Successfully ported growth"),
        Err(err) => {
            println!("{err:?}");
            exit(1)
        }
    };

    log::info!("Porting Activities");
    let activities_file_in = dirs.logs_dir_in.join(ACTIVITIES_CSV);
    let activities_file_out = dirs.logs_dir_out.join(ACTIVITIES_CSV);
    match <Vec<LogCSV> as Port<Vec<LogItem>>>::port(&activities_file_in, &(), &activities_file_out)
    {
        Ok(()) => println!("Successfully ported activities"),
        Err(err) => {
            println!("{err:?}");
            exit(1)
        }
    }

    log::info!("Porting Images");
    let images_dir_in = dirs.data_dir_in.join("img");
    match <Vec<OldImage> as Port<Vec<PlantImage>>>::port(
        &images_dir_in,
        &(dirs.plants_dir_out, "%d%m%Y".to_owned()),
        &PathBuf::new(),
    ) {
        Ok(()) => println!("Successfully ported images"),
        Err(err) => {
            println!("{err:?}");
            exit(1)
        }
    }

    log::info!("Porting Locations");
    let locations_file_out = dirs.data_dir_out.join(LOCATIONS_CSV);
    match <Vec<PlantJSON> as Port<Vec<Location>>>::port(
        &dirs.plants_dir_in,
        &INTERACTIVE,
        &locations_file_out,
    ) {
        Ok(()) => println!("Successfully ported locations"),
        Err(err) => {
            println!("{err:?}");
            exit(1)
        }
    }
}
