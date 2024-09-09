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
    use super::shared::plant_link::PlantLink;
    use chrono::NaiveDate;
    use plants::{
        graveyard::GraveyardPlant,
        growth_item::GrowthItem,
        log_item::LogItem,
        plant::{Plant, PlantInfo, PlantLocation, PlantSpecies},
        species::{Species, SunlightRequirement},
    };

    pub const DATE_FORMAT: &str = "%d.%m.%Y";

    pub fn sample_date1() -> NaiveDate {
        NaiveDate::parse_from_str("01.01.1970", DATE_FORMAT).unwrap()
    }

    pub fn sample_date2() -> NaiveDate {
        NaiveDate::parse_from_str("02.01.1970", DATE_FORMAT).unwrap()
    }

    pub fn sample_date3() -> NaiveDate {
        NaiveDate::parse_from_str("03.01.1970", DATE_FORMAT).unwrap()
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

    fn example_growth1() -> GrowthItem {
        GrowthItem {
            plant: "Plant1".to_owned(),
            date: sample_date2(),
            height_cm: 100.0,
            width_cm: 75.0,
            note: None,
            health: 3,
        }
    }

    fn example_growth4() -> GrowthItem {
        GrowthItem {
            plant: "Plant1".to_owned(),
            date: sample_date1(),
            height_cm: 50.0,
            width_cm: 25.0,
            note: None,
            health: 3,
        }
    }

    fn example_growth2() -> GrowthItem {
        GrowthItem {
            plant: "Plant2".to_owned(),
            date: sample_date2(),
            height_cm: 75.3,
            width_cm: 98.5,
            note: None,
            health: 3,
        }
    }

    fn example_growth5() -> GrowthItem {
        GrowthItem {
            plant: "Plant2".to_owned(),
            date: sample_date1(),
            height_cm: 24.0,
            width_cm: 18.3,
            note: None,
            health: 3,
        }
    }

    fn example_growth3() -> GrowthItem {
        GrowthItem {
            plant: "Plant3".to_owned(),
            date: sample_date2(),
            height_cm: 34.2,
            width_cm: 83.4,
            note: None,
            health: 3,
        }
    }

    fn example_growth6() -> GrowthItem {
        GrowthItem {
            plant: "Plant3".to_owned(),
            date: sample_date1(),
            height_cm: 2.0,
            width_cm: 7.3,
            note: None,
            health: 3,
        }
    }

    pub fn example_species() -> Species {
        Species {
            name: "test species".to_owned(),
            scientific_name: "dummy".to_owned(),
            genus: "dummy".to_owned(),
            family: "dummy".to_owned(),
            planting_distance: None,
            sunlight: SunlightRequirement::Direct,
            temp_min: 0.0,
            temp_max: 100.0,
            opt_temp_min: 0.0,
            opt_temp_max: 100.0,
            ph_min: 0.0,
            ph_max: 10.0,
            watering_notes: vec![],
            avg_watering_days: Some(1),
            avg_fertilizing_days: Some(1),
            fertilizing_notes: vec![],
            pruning_notes: vec![],
            companions: vec![],
            additional_notes: vec![],
        }
    }

    pub fn example_plant1() -> Plant {
        let name = "Plant1".to_owned();
        Plant {
            info: PlantInfo {
                name: name.clone(),
                species: PlantSpecies::Species(Box::new(example_species())),
                location: PlantLocation::Other("test location".to_owned()),
                origin: "test origin".to_owned(),
                obtained: sample_date1(),
                auto_water: false,
                notes: vec![],
            },
            growth: vec![example_growth4(), example_growth1()],
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
                species: PlantSpecies::Species(Box::new(example_species())),
                location: PlantLocation::Other("test location".to_owned()),
                origin: "test origin".to_owned(),
                obtained: sample_date2(),
                auto_water: true,
                notes: vec![],
            },
            growth: vec![example_growth5(), example_growth2()],
            activities: vec![example_activity1(name, "a second note".to_owned())],
            images: vec![],
        }
    }

    pub fn example_plant3() -> Plant {
        let name = "Plant3".to_owned();
        Plant {
            info: PlantInfo {
                name: name.clone(),
                species: PlantSpecies::Species(Box::new(example_species())),
                location: PlantLocation::Other("test location".to_owned()),
                origin: "test origin".to_owned(),
                obtained: sample_date3(),
                auto_water: false,
                notes: vec![],
            },
            growth: vec![example_growth6(), example_growth3()],
            activities: vec![
                example_activity1(name.clone(), "a note".to_owned()),
                example_activity2(name),
            ],
            images: vec![],
        }
    }

    pub fn example_graveyard_plant1() -> GraveyardPlant {
        GraveyardPlant {
            name: "Plant1".to_owned(),
            species: "test species".to_owned(),
            planted: sample_date1(),
            died: sample_date2(),
            reason: "testing".to_owned(),
        }
    }

    pub fn example_graveyard_plant2() -> GraveyardPlant {
        GraveyardPlant {
            name: "Plant2".to_owned(),
            species: "test species".to_owned(),
            planted: sample_date1(),
            died: sample_date2(),
            reason: "testing".to_owned(),
        }
    }

    pub fn example_plantlink1() -> PlantLink {
        (&example_plant1(), "plants").into()
    }

    pub fn example_plantlink2() -> PlantLink {
        (&example_plant2(), "plants").into()
    }

    pub fn example_plantlink3() -> PlantLink {
        (&example_plant3(), "plants").into()
    }
}
