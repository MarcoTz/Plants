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
        let mut locations: Vec<Location> = vec![];
        for plant in self.iter() {
            if locations
                .iter()
                .map(|loc| loc.name.clone())
                .collect::<Vec<String>>()
                .contains(&plant.current_location)
            {
                continue;
            }
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
                    .map_err(|_| Error::Input("location outside".to_owned()))?;
                let outside_bool = if outside.trim().to_lowercase() == "y" {
                    Ok(true)
                } else if outside.trim().to_lowercase() == "n" {
                    Ok(false)
                } else {
                    Err(Error::Parse("boolean (location outside".to_owned()))
                }?;
                new_location.outside = outside_bool;
            }
            locations.push(new_location);
        }
        Ok(locations)
    }

    fn save_new(locations: Vec<Location>, location_file: &Self::SaveArgs) -> Result<(), Error> {
        log::info!("Saving new Locations");
        write_csv(locations, location_file, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod locations_test {
    use super::{Location, PlantJSON, Port};
    use crate::port::test_common::{
        example_plant_json1, example_plant_json2, example_plant_json3, BASE_DIR, LOCATION_FILE,
        PLANTS_DIR_IN,
    };
    use database::file_backend::load_csv::load_csv;
    use std::collections::HashSet;
    use std::path::PathBuf;

    fn example_location1() -> Location {
        Location {
            name: "Location1".to_owned(),
            outside: false,
        }
    }
    fn example_location2() -> Location {
        Location {
            name: "Location2".to_owned(),
            outside: false,
        }
    }

    #[test]
    fn load_old() {
        let plants_dir = PathBuf::from(BASE_DIR).join(PLANTS_DIR_IN);
        let result = HashSet::from_iter(
            <Vec<PlantJSON> as Port<Vec<Location>>>::load_old(&plants_dir)
                .unwrap()
                .iter()
                .cloned(),
        );
        let expected = HashSet::from([
            example_plant_json3(),
            example_plant_json2(),
            example_plant_json1(),
        ]);
        assert_eq!(result, expected)
    }

    #[test]
    fn convert() {
        let result: Vec<Location> = vec![
            example_plant_json1(),
            example_plant_json2(),
            example_plant_json3(),
        ]
        .convert(&false)
        .unwrap();
        let expected = vec![example_location1(), example_location2()];
        assert_eq!(result, expected)
    }

    #[test]
    fn save_new() {
        let loc_file = PathBuf::from(BASE_DIR).join(LOCATION_FILE);

        if loc_file.exists() {
            std::fs::remove_file(loc_file.clone()).unwrap();
        }
        assert!(!loc_file.exists());

        <Vec<PlantJSON> as Port<Vec<Location>>>::save_new(
            vec![example_location1(), example_location2()],
            &loc_file,
        )
        .unwrap();

        assert!(loc_file.exists());
        let result: Vec<Location> = load_csv(&loc_file).unwrap();
        let expected = vec![example_location1(), example_location2()];
        assert_eq!(result, expected);

        std::fs::remove_file(loc_file.clone()).unwrap();
        assert!(!loc_file.exists())
    }
}
