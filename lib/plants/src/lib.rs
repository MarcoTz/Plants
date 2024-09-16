pub mod errors;
pub mod graveyard;
pub mod growth_item;
pub mod location;
pub mod log_item;
pub mod named;
pub mod plant;
pub mod plant_update;
pub mod serialize;
pub mod species;
pub mod species_update;

#[cfg(test)]
pub mod test_common {
    use super::{
        growth_item::GrowthItem,
        location::Location,
        log_item::LogItem,
        plant::{Plant, PlantImage, PlantInfo, PlantLocation, PlantSpecies},
        species::{Species, SunlightRequirement},
    };
    use chrono::NaiveDate;
    use std::path::PathBuf;

    pub fn example_species() -> Species {
        Species {
            name: "Test species".to_owned(),
            scientific_name: "Scientific Name".to_owned(),
            genus: "Genus".to_owned(),
            family: "Family".to_owned(),
            sunlight: SunlightRequirement::Direct,
            temp_min: 0.0,
            temp_max: 30.0,
            opt_temp_min: 10.0,
            opt_temp_max: 25.0,
            planting_distance: Some(30.0),
            ph_min: 4.5,
            ph_max: 8.5,
            watering_notes: vec![],
            avg_watering_days: Some(7),
            fertilizing_notes: vec![],
            avg_fertilizing_days: Some(14),
            pruning_notes: vec![],
            companions: vec![],
            additional_notes: vec![],
        }
    }

    pub fn example_date1() -> NaiveDate {
        NaiveDate::parse_from_str("01.01.1970", "%d.%m.%Y").unwrap()
    }

    pub fn example_date2() -> NaiveDate {
        NaiveDate::parse_from_str("02.01.1970", "%d.%m.%Y").unwrap()
    }

    pub fn example_location() -> Location {
        Location {
            name: "Inside".to_owned(),
            outside: false,
        }
    }

    pub fn example_plant_info() -> PlantInfo {
        PlantInfo {
            name: "A Plant".to_owned(),
            species: PlantSpecies::Species(Box::new(example_species())),
            location: PlantLocation::Location(Box::new(example_location())),
            origin: "An Origin".to_owned(),
            obtained: example_date1(),
            auto_water: false,
            notes: vec![],
        }
    }

    pub fn example_growth1() -> GrowthItem {
        GrowthItem {
            plant: "A Plant".to_owned(),
            date: example_date1(),
            height_cm: 10.0,
            width_cm: 10.0,
            note: None,
            health: 3,
        }
    }

    pub fn example_growth2() -> GrowthItem {
        GrowthItem {
            plant: "A Plant".to_owned(),
            date: example_date2(),
            height_cm: 15.0,
            width_cm: 15.0,
            note: None,
            health: 4,
        }
    }

    pub fn example_activity1() -> LogItem {
        LogItem {
            activity: "Watering".to_owned(),
            date: example_date1(),
            plant: "A Plant".to_owned(),
            note: None,
        }
    }

    pub fn example_activity2() -> LogItem {
        LogItem {
            activity: "Fertilizing".to_owned(),
            date: example_date2(),
            plant: "A Plant".to_owned(),
            note: None,
        }
    }

    pub fn example_image1() -> PlantImage {
        PlantImage {
            created: example_date1(),
            file_name: "01011970.jpg".to_owned(),
            file_path: PathBuf::from("./"),
        }
    }

    pub fn example_image2() -> PlantImage {
        PlantImage {
            created: example_date2(),
            file_name: "02011970.jpg".to_owned(),
            file_path: PathBuf::from("/"),
        }
    }

    pub fn example_plant() -> Plant {
        Plant {
            info: example_plant_info(),
            growth: vec![example_growth1(), example_growth2()],
            activities: vec![example_activity1(), example_activity2()],
            images: vec![example_image1(), example_image2()],
        }
    }

    pub fn empty_plant() -> Plant {
        let mut info = example_plant_info();
        info.species = PlantSpecies::Other("another species".to_owned());
        info.location = PlantLocation::Other("another location".to_owned());
        Plant {
            info,
            growth: vec![],
            activities: vec![],
            images: vec![],
        }
    }

    pub fn example_plant2() -> Plant {
        let mut plant2 = example_plant();
        plant2.info.obtained = example_date2();
        let mut growth1 = example_growth1();
        growth1.height_cm = 5.0;
        growth1.width_cm = 5.0;
        let mut growth2 = example_growth2();
        growth2.height_cm = 12.0;
        growth2.width_cm = 12.0;
        plant2.growth = vec![growth1, growth2];
        plant2
    }
}
