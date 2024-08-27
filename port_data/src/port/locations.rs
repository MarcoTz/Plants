use super::{plants::PlantJSON, Port};
use crate::errors::Error;
use database::file_backend::write_csv::write_csv;
use plants::{location::Location, plant::PlantInfo};
use std::{io, path::PathBuf};

impl Port<Vec<Location>> for Vec<PlantJSON> {
    type LoadArgs = PathBuf;
    type SaveArgs = PathBuf;
    type ConvertArgs = bool;

    fn load_old(args: &Self::LoadArgs) -> Result<Self, Error> {
        log::info!("Loading old Locations");
        <Vec<PlantJSON> as Port<Vec<PlantInfo>>>::load_old(args)
    }

    fn convert(self, interactive: &Self::ConvertArgs) -> Result<Vec<Location>, Error> {
        log::info!("Converting Locations");
        let mut locations = vec![];
        for plant in self.iter() {
            let mut new_location = Location {
                name: plant.current_location.clone(),
                outside: false,
            };

            if interactive.to_owned() {
                let stdin = io::stdin();
                let mut outside = String::new();
                println!(
                    "Please enter if {} is outside (y/n))",
                    plant.current_location
                );
                stdin
                    .read_line(&mut outside)
                    .map_err(|_| Error::InputErr("location outside".to_owned()))?;
                let outside_bool = if outside.trim().to_lowercase() == "y" {
                    Ok(true)
                } else if outside.trim().to_lowercase() == "n" {
                    Ok(false)
                } else {
                    Err(Error::ParseError("boolean (location outside".to_owned()))
                }?;
                new_location.outside = outside_bool;
            }
            locations.push(new_location);
        }
        Ok(locations)
    }

    fn save_new(locations: Vec<Location>, location_file: &Self::SaveArgs) -> Result<(), Error> {
        log::info!("Saving new Locations");
        write_csv(locations, location_file)?;
        Ok(())
    }
}
