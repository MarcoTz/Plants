use database::{database_manager::DatabaseManager, file_backend::FileDB, sqlite_backend::SQLiteDB};
use std::path::PathBuf;

type SourceDB = FileDB;
type TargetDB = SQLiteDB;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = SourceDB::default();
    let mut target = TargetDB::new(PathBuf::from("plants.db")).map_err(|err| err.to_string())?;

    // sync plants
    let all_plants = source.get_all_plants()?;
    println!("Loaded {} plants from source", all_plants.len());
    target.write_plants(all_plants.into_iter().map(|pl| pl.info).collect())?;
    println!("Write plants to target");
    let target_num = target.get_num_plants()?;
    println!("Target now has {target_num} plants");

    // sync species
    let all_species = source.get_all_species()?;
    println!("Loaded {} species from source", all_species.len());
    for species in all_species.into_iter() {
        target.write_species(species)?;
    }
    println!("Wrote species to taget");
    let target_num = target.get_all_species()?.len();
    println!("Target now has {target_num} plants");

    let graveyard = source.get_graveyard()?;
    println!("Loaded {} graveyard plants from source", graveyard.len());
    for graveyard_plant in graveyard.into_iter() {
        target.add_to_graveyard(graveyard_plant)?;
    }
    println!("Wrote graveyard plants to target");
    let target_num = target.get_graveyard()?.len();
    println!("Target now has {target_num} graveyard plants");

    let locations = source.get_locations()?;
    println!("Loaded {} locations from source", locations.len());
    for loc in locations.into_iter() {
        target.write_location(loc)?;
    }
    println!("Wrote locations to target");
    let target_num = target.get_locations()?.len();
    println!("Target now has {target_num} locations");

    let logs = source.get_logs()?;
    println!("Loaded {} logs from source", logs.len());
    target.write_logs(logs)?;
    println!("Wrote logs to target");
    let target_num = target.get_locations()?.len();
    println!("Target now has {target_num} locations");

    let growth = source.get_growth()?;
    println!("Loaded {} growth items from source", growth.len());
    target.write_growths(growth)?;
    println!("Wrote growth to target");
    let target_num = target.get_growth()?.len();
    println!("Target now has {target_num} growth items");

    Ok(())
}
