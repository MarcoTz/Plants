pub mod activities;
pub mod growth;
pub mod images;
pub mod locations;
pub mod plants;
pub mod species;
use crate::errors::Error;

pub trait Port<U>: Sized {
    type LoadArgs;
    type SaveArgs;
    type ConvertArgs;

    fn load_old(args: &Self::LoadArgs) -> Result<Self, Error>;
    fn convert(self, args: &Self::ConvertArgs) -> Result<U, Error>;
    fn save_new(new: U, args: &Self::SaveArgs) -> Result<(), Error>;

    fn port(
        load_args: &Self::LoadArgs,
        convert_args: &Self::ConvertArgs,
        save_args: &Self::SaveArgs,
    ) -> Result<(), Error> {
        let old = Self::load_old(load_args)?;
        let new = old.convert(convert_args)?;
        Self::save_new(new, save_args)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test_common {
    use super::plants::{BoolOrString, PlantJSON};
    use chrono::NaiveDate;

    pub const BASE_DIR: &str = "../testing/port";
    pub const LOGS_FILE_IN: &str = "Activities.csv";
    pub const LOGS_FILE_OUT: &str = "Activities_new.csv";
    pub const GROWTH_FILE_IN: &str = "Growth.csv";
    pub const GROWTH_FILE_OUT: &str = "Growth_new.csv";
    pub const IMAGES_DIR: &str = "images";
    pub const PLANTS_DIR_IN: &str = "plants";
    pub const PLANTS_DIR_OUT: &str = "plants_new";
    pub const LOCATION_FILE: &str = "locations.csv";
    pub const SPECIES_DIR_IN: &str = "species";
    pub const SPECIES_DIR_OUT: &str = "species_new";

    pub fn example_date1() -> NaiveDate {
        NaiveDate::parse_from_str("01.01.1970", "%d.%m.%Y").unwrap()
    }

    pub fn example_date2() -> NaiveDate {
        NaiveDate::parse_from_str("02.01.1970", "%d.%m.%Y").unwrap()
    }

    pub fn example_plant_json1() -> PlantJSON {
        PlantJSON {
            auto_watering: BoolOrString::Bool(false),
            current_location: "Location1".to_owned(),
            obtained: "01.01.1970".to_owned(),
            origin: "test origin".to_owned(),
            plant_health: "3".to_owned(),
            plant_name: "Plant1".to_owned(),
            plant_notes: vec![],
            species_name: "Species1".to_owned(),
        }
    }

    pub fn example_plant_json2() -> PlantJSON {
        PlantJSON {
            auto_watering: BoolOrString::Str("False".to_owned()),
            current_location: "Location2".to_owned(),
            obtained: "01.01.1970".to_owned(),
            origin: "test origin".to_owned(),
            plant_health: "4".to_owned(),
            plant_name: "Plant2".to_owned(),
            plant_notes: vec![],
            species_name: "Species1".to_owned(),
        }
    }

    pub fn example_plant_json3() -> PlantJSON {
        PlantJSON {
            auto_watering: BoolOrString::Bool(true),
            current_location: "Location1".to_owned(),
            obtained: "02.01.1970".to_owned(),
            origin: "test origin".to_owned(),
            plant_health: "3".to_owned(),
            plant_name: "Plant3".to_owned(),
            plant_notes: vec![],
            species_name: "Species2".to_owned(),
        }
    }
}
