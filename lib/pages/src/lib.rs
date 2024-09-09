pub mod activities;
pub mod css;
pub mod errors;
pub mod gallery;
pub mod graveyard;
pub mod index;
pub mod page;
pub mod plant_details;
pub mod plant_overview;
pub mod shared;
pub mod species_details;
pub mod species_overview;

#[cfg(test)]
pub mod test_common {
    use chrono::NaiveDate;
    use plants::{
        log_item::LogItem,
        plant::{Plant, PlantInfo, PlantLocation, PlantSpecies},
    };

    pub const DATE_FORMAT: &str = "%d.%m.%Y";

    pub fn sample_date1() -> NaiveDate {
        NaiveDate::parse_from_str("01.01.1970", DATE_FORMAT).unwrap()
    }

    pub fn sample_date2() -> NaiveDate {
        NaiveDate::parse_from_str("02.01.1970", DATE_FORMAT).unwrap()
    }

    fn example_activity1(plant: String, note: String) -> LogItem {
        LogItem {
            activity: "Watering".to_owned(),
            date: sample_date1(),
            plant,
            note: Some(note),
        }
    }

    fn example_activity2(plant: String) -> LogItem {
        LogItem {
            activity: "Fertilizing".to_owned(),
            date: sample_date2(),
            plant,
            note: Some("a different note".to_owned()),
        }
    }

    pub fn example_plant1() -> Plant {
        let name = "Plant1".to_owned();
        Plant {
            info: PlantInfo {
                name: name.clone(),
                species: PlantSpecies::Other("test species".to_owned()),
                location: PlantLocation::Other("test location".to_owned()),
                origin: "test origin".to_owned(),
                obtained: sample_date1(),
                auto_water: false,
                notes: vec![],
            },
            growth: vec![],
            activities: vec![
                example_activity1(name.clone(), "a note".to_owned()),
                example_activity2(name),
            ],
            images: vec![],
        }
    }

    pub fn example_plant2() -> Plant {
        let name = "Plant2".to_owned();

        Plant {
            info: PlantInfo {
                name: name.clone(),
                species: PlantSpecies::Other("test species".to_owned()),
                location: PlantLocation::Other("test location".to_owned()),
                origin: "test origin".to_owned(),
                obtained: sample_date1(),
                auto_water: false,
                notes: vec![],
            },
            growth: vec![],
            activities: vec![example_activity1(name, "a second note".to_owned())],
            images: vec![],
        }
    }

    pub fn example_plant3() -> Plant {
        let name = "Plant3".to_owned();
        Plant {
            info: PlantInfo {
                name: name.clone(),
                species: PlantSpecies::Other("test species".to_owned()),
                location: PlantLocation::Other("test location".to_owned()),
                origin: "test origin".to_owned(),
                obtained: sample_date1(),
                auto_water: false,
                notes: vec![],
            },
            growth: vec![],
            activities: vec![
                example_activity1(name.clone(), "a note".to_owned()),
                example_activity2(name),
            ],
            images: vec![],
        }
    }
}
