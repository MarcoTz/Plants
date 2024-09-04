pub mod errors;
pub mod json_to_plant;
pub mod load_csv;
pub mod load_json;
pub mod write_csv;
pub mod write_json;

use errors::Error;
use json_to_plant::load_plants;
use load_csv::{load_graveyard, load_locations};
use load_json::load_species;
use write_csv::{write_activities, write_graveyard, write_growth};
use write_json::{write_plants, write_species};

use crate::database_manager::DatabaseManager;
use plants::{
    graveyard::GraveyardPlant,
    growth_item::GrowthItem,
    location::Location,
    log_item::LogItem,
    named::Named,
    plant::{Plant, PlantInfo, PlantSpecies},
    species::Species,
};
use std::{
    fs::{remove_file, rename},
    io::Error as IOError,
    path::PathBuf,
};

#[derive(Debug, PartialEq)]
pub struct FileDB {
    pub plants_dir: PathBuf,
    pub species_dir: PathBuf,
    pub location_file: PathBuf,
    pub logs_dir: PathBuf,
    pub graveyard_csv: String,
    pub growth_csv: String,
    pub activities_csv: String,
    pub date_format: String,

    pub plants_cache: Vec<Plant>,
    pub graveyard_cache: Vec<GraveyardPlant>,
    pub species_cache: Vec<Species>,
    pub location_cache: Vec<Location>,
}

impl Default for FileDB {
    fn default() -> Self {
        let data_dir: PathBuf = "data".into();
        FileDB {
            plants_dir: data_dir.join("Plants"),
            species_dir: data_dir.join("Species"),
            location_file: data_dir.join("Locations.csv"),
            logs_dir: data_dir.join("Logs"),
            graveyard_csv: "Graveyard.csv".to_owned(),
            growth_csv: "Growth.csv".to_owned(),
            activities_csv: "Activities.csv".to_owned(),
            date_format: "%d.%m.%Y".to_owned(),
            plants_cache: vec![],
            graveyard_cache: vec![],
            species_cache: vec![],
            location_cache: vec![],
        }
    }
}

impl FileDB {
    pub fn get_activities_filepath(&self) -> PathBuf {
        self.logs_dir.join(self.activities_csv.clone())
    }

    pub fn get_graveyard_filepath(&self) -> PathBuf {
        self.logs_dir.join(self.graveyard_csv.clone())
    }

    pub fn get_growth_filepath(&self) -> PathBuf {
        self.logs_dir.join(self.growth_csv.clone())
    }

    fn load_plants(&mut self) -> Result<(), Error> {
        log::info!("Loading plants from json and csv");
        let activity_file = self.get_activities_filepath();
        let growth_file = self.get_growth_filepath();
        let plants = load_plants(
            &self.plants_dir,
            &self.species_dir,
            &activity_file,
            &growth_file,
            &self.location_file,
        )?;
        self.plants_cache = plants;
        Ok(())
    }

    fn load_species(&mut self) -> Result<(), Error> {
        log::info!("Loading species from json");
        let species = load_species(&self.species_dir)?;
        self.species_cache = species;
        Ok(())
    }

    fn load_graveyard(&mut self) -> Result<(), Error> {
        log::info!("Loading graveyard from csv");
        let graveyard_file = self.get_graveyard_filepath();
        let graveyard = load_graveyard(&graveyard_file)?;
        self.graveyard_cache = graveyard;
        Ok(())
    }

    fn load_locations(&mut self) -> Result<(), Error> {
        log::info!("Loading Locations from csv");
        let locations = load_locations(&self.location_file)?;
        self.location_cache = locations;
        Ok(())
    }
}

impl DatabaseManager for FileDB {
    fn get_all_plants(&mut self) -> Result<Vec<Plant>, Box<dyn std::error::Error>> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }
        Ok(self.plants_cache.clone())
    }

    fn get_num_plants(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }
        Ok(self.plants_cache.len() as i32)
    }

    fn get_plant(&mut self, plant_name: &str) -> Result<Plant, Box<dyn std::error::Error>> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }
        self.plants_cache
            .iter()
            .filter(|pl| pl.info.name == plant_name)
            .cloned()
            .collect::<Vec<Plant>>()
            .first()
            .cloned()
            .ok_or(Error::PlantNotFound(plant_name.to_owned()).into())
    }

    fn get_all_species(&mut self) -> Result<Vec<Species>, Box<dyn std::error::Error>> {
        if self.species_cache.is_empty() {
            self.load_species()?;
        }
        Ok(self.species_cache.clone())
    }

    fn get_species(&mut self, species_name: &str) -> Result<Species, Box<dyn std::error::Error>> {
        if self.species_cache.is_empty() {
            self.load_species()?;
        }
        self.species_cache
            .iter()
            .filter(|sp| sp.name == species_name)
            .cloned()
            .collect::<Vec<Species>>()
            .first()
            .cloned()
            .ok_or(Error::SpeciesNotFound(species_name.to_owned()).into())
    }

    fn get_graveyard(&mut self) -> Result<Vec<GraveyardPlant>, Box<dyn std::error::Error>> {
        if self.graveyard_cache.is_empty() {
            self.load_graveyard()?;
        }
        Ok(self.graveyard_cache.clone())
    }

    fn get_plants_species(
        &mut self,
        species_name: &str,
    ) -> Result<Vec<Plant>, Box<dyn std::error::Error>> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }
        let species_plants = self
            .plants_cache
            .iter()
            .filter(|plant| match &plant.info.species {
                PlantSpecies::Other(name) => name == species_name,
                PlantSpecies::Species(sp) => sp.name == species_name,
            })
            .cloned()
            .collect();
        Ok(species_plants)
    }

    fn get_locations(&mut self) -> Result<Vec<Location>, Box<dyn std::error::Error>> {
        if self.location_cache.is_empty() {
            self.load_locations()?;
        }
        Ok(self.location_cache.clone())
    }

    fn get_location(
        &mut self,
        location_name: &str,
    ) -> Result<Location, Box<dyn std::error::Error>> {
        if self.location_cache.is_empty() {
            self.load_locations()?;
        }
        let err = Error::LocationNotFound(location_name.to_owned()).into();
        self.location_cache
            .iter()
            .filter(|loc| loc.get_name() == location_name)
            .cloned()
            .collect::<Vec<Location>>()
            .first()
            .cloned()
            .ok_or(err)
    }

    fn plant_exists(&mut self, plant_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }

        Ok(self
            .plants_cache
            .iter()
            .any(|pl| pl.info.name == plant_name))
    }

    fn species_exists(&mut self, species_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        if self.species_cache.is_empty() {
            self.load_species()?;
        }
        Ok(self.species_cache.iter().any(|sp| sp.name == species_name))
    }

    fn write_logs(&mut self, logs: Vec<LogItem>) -> Result<(), Box<dyn std::error::Error>> {
        write_activities(logs, &self.get_activities_filepath())?;
        Ok(())
    }

    fn get_plants_by_location(
        &mut self,
        location: &str,
    ) -> Result<Vec<Plant>, Box<dyn std::error::Error>> {
        if self.plants_cache.is_empty() {
            self.load_plants()?;
        }

        Ok(self
            .plants_cache
            .iter()
            .filter(|pl| pl.info.location.get_name() == location)
            .cloned()
            .collect())
    }

    fn write_growths(&mut self, growth: Vec<GrowthItem>) -> Result<(), Box<dyn std::error::Error>> {
        write_growth(growth, &self.get_growth_filepath())?;
        Ok(())
    }

    fn write_plant(&mut self, plant: PlantInfo) -> Result<(), Box<dyn std::error::Error>> {
        write_plants(vec![plant], &self.plants_dir)?;
        Ok(())
    }

    fn write_species(&mut self, species: Species) -> Result<(), Box<dyn std::error::Error>> {
        write_species(vec![species], &self.species_dir)?;
        Ok(())
    }

    fn kill_plant(&mut self, plant: GraveyardPlant) -> Result<(), Box<dyn std::error::Error>> {
        let name = plant.name.clone();
        write_graveyard(vec![plant], &self.get_graveyard_filepath())?;
        let plant_filename = name.replace(' ', "") + ".json";
        let plant_path = PathBuf::from(&self.plants_dir).join(plant_filename.clone());
        remove_file(plant_path.clone()).map_err(<IOError as Into<Error>>::into)?;
        self.plants_cache = self
            .plants_cache
            .iter()
            .filter(|pl| pl.info.name == name)
            .cloned()
            .collect();
        let dead_path = PathBuf::from("dead").join(name);
        rename(plant_path, dead_path).map_err(<IOError as Into<Error>>::into)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test_common {
    use chrono::NaiveDate;
    use plants::{
        graveyard::GraveyardPlant,
        growth_item::GrowthItem,
        location::Location,
        log_item::LogItem,
        plant::{Plant, PlantImage, PlantInfo, PlantLocation, PlantSpecies},
        species::{Species, SunlightRequirement},
    };
    use std::path::{Path, PathBuf};

    pub const TESTING_BASE: &str = "../../testing";

    pub const CSV_DUMMY: &str = "../../testing/dummy.csv";
    pub const CSV_DUMMY_BAD_LINES: &str = "../../testing/dummy_badlines.csv";
    pub const CSV_DUMMY_DESERIALIZE: &str = "../../testing/dummy_deserialize.csv";
    pub const GRAVEYARD_DUMMY: &str = "../../testing/Logs/Graveyard.csv";
    pub const ACTIVITIES_DUMMY: &str = "../../testing/Logs/Activities.csv";
    pub const GROWTH_DUMMY: &str = "../../testing/Logs/Growth.csv";
    pub const LOCATIONS_DUMMY: &str = "../../testing/Locations.csv";

    pub const JSON_DUMMY: &str = "../../testing/dummy.json";
    pub const JSON_DUMMY_DESERIALIZE: &str = "../../testing/dummy_deserialize.json";
    pub const JSON_DUMMY_DIR: &str = "../../testing/json_dir";
    pub const JSON_DUMMY_NO_SUBDIR: &str = "../../testing/json_dir_no_subdir";
    pub const JSON_DUMMY_DIR_BAD_JSON: &str = "../../testing/json_dir_bad_json";

    pub const DUMMY_PLANT_PATH: &str = "../../testing/plants/";
    pub const DUMMY_SPECIES_PATH: &str = "../../testing/species/";
    pub const DUMMY_LOGS_PATH: &str = "../../testing/Logs/";

    pub const FILE_DOES_NOT_EXIST: &str = "../../testing/notaflie";

    pub fn dummy_date() -> NaiveDate {
        NaiveDate::parse_from_str("01.01.1970", "%d.%m.%Y").unwrap()
    }

    pub fn dummy_species() -> Species {
        Species {
            name: "test species".to_owned(),
            scientific_name: "testing name".to_owned(),
            genus: "testing genus".to_owned(),
            family: "testing family".to_owned(),
            sunlight: SunlightRequirement::Direct,
            temp_min: 0.0,
            temp_max: 0.0,
            opt_temp_min: 0.0,
            opt_temp_max: 0.0,
            planting_distance: Some(0.0),
            ph_min: 0.0,
            ph_max: 0.0,
            watering_notes: vec!["".to_owned()],
            avg_watering_days: Some(0),
            fertilizing_notes: vec!["".to_owned()],
            avg_fertilizing_days: Some(0),
            pruning_notes: vec!["".to_owned()],
            companions: vec!["".to_owned()],
            additional_notes: vec![],
        }
    }

    pub fn dummy_location1() -> Location {
        Location {
            name: "test outside".to_owned(),
            outside: true,
        }
    }

    pub fn dummy_location2() -> Location {
        Location {
            name: "test inside".to_owned(),
            outside: false,
        }
    }

    pub fn dummy_location3() -> Location {
        Location {
            name: "test location".to_owned(),
            outside: false,
        }
    }

    pub fn dummy_graveyard1() -> GraveyardPlant {
        GraveyardPlant {
            name: "Dummy1".to_owned(),
            species: "test species".to_owned(),
            planted: dummy_date(),
            died: NaiveDate::parse_from_str("02.01.1970", "%d.%m.%Y").unwrap(),
            reason: "testing".to_owned(),
        }
    }

    pub fn dummy_graveyard2() -> GraveyardPlant {
        GraveyardPlant {
            name: "Dummy2".to_owned(),
            species: "testing species".to_owned(),
            planted: dummy_date(),
            died: NaiveDate::parse_from_str("02.01.1970", "%d.%m.%Y").unwrap(),
            reason: "testing".to_owned(),
        }
    }

    pub fn dummy_plant1() -> Plant {
        Plant {
            info: PlantInfo {
                name: "Dummy1".to_owned(),
                species: PlantSpecies::Species(Box::new(dummy_species())),
                location: PlantLocation::Location(Box::new(dummy_location3())),
                origin: "test origin".to_owned(),
                obtained: dummy_date(),
                auto_water: true,
                notes: vec![],
            },
            growth: vec![GrowthItem {
                plant: "Dummy1".to_owned(),
                date: dummy_date(),
                height_cm: 10.0,
                width_cm: 10.0,
                note: None,
                health: 3,
            }],
            activities: vec![LogItem {
                activity: "Watering".to_owned(),
                date: dummy_date(),
                plant: "Dummy1".to_owned(),
                note: None,
            }],
            images: vec![PlantImage {
                created: dummy_date(),
                file_name: "01011970.jpg".to_owned(),
                file_path: PathBuf::from("../../testing/plants/Dummy1"),
            }],
        }
    }

    pub fn dummy_plant2() -> Plant {
        Plant {
            info: PlantInfo {
                name: "Dummy2".to_owned(),
                species: PlantSpecies::Other("a different species".to_owned()),
                location: PlantLocation::Other("a different test location".to_owned()),
                origin: "test origin".to_owned(),
                obtained: dummy_date(),
                auto_water: true,
                notes: vec![],
            },
            growth: vec![],
            activities: vec![],
            images: vec![],
        }
    }

    #[test]
    fn check_dummy_date() {
        dummy_date();
    }

    #[test]
    fn ensure_base_exists() {
        let base_dir = Path::new(TESTING_BASE);
        assert!(base_dir.exists());
        assert!(base_dir.is_dir());
    }

    #[test]
    fn ensure_csv_exists() {
        let csv_dummy = Path::new(CSV_DUMMY);
        let csv_dummy_bad_lines = Path::new(CSV_DUMMY_BAD_LINES);
        let csv_dummy_deserialize = Path::new(CSV_DUMMY_DESERIALIZE);
        assert!(csv_dummy.exists());
        assert!(csv_dummy.is_file());
        assert!(csv_dummy_bad_lines.exists());
        assert!(csv_dummy_bad_lines.is_file());
        assert!(csv_dummy_deserialize.exists());
        assert!(csv_dummy_deserialize.is_file());
    }

    #[test]
    fn ensure_logs_exist() {
        let graveyard = Path::new(GRAVEYARD_DUMMY);
        let activities = Path::new(ACTIVITIES_DUMMY);
        let growth = Path::new(GROWTH_DUMMY);
        let locations = Path::new(LOCATIONS_DUMMY);
        assert!(graveyard.exists());
        assert!(graveyard.is_file());
        assert!(activities.exists());
        assert!(activities.is_file());
        assert!(growth.exists());
        assert!(growth.is_file());
        assert!(locations.exists());
        assert!(locations.is_file());
    }

    #[test]
    fn ensure_does_not_exist() {
        assert!(!Path::new(FILE_DOES_NOT_EXIST).exists());
    }

    #[test]
    fn ensure_json_exists() {
        let json_dummy = Path::new(JSON_DUMMY);
        let json_dummy_deserialize = Path::new(JSON_DUMMY_DESERIALIZE);
        let json_dummy_dir = Path::new(JSON_DUMMY_DIR);
        let json_dummy_no_subdir = Path::new(JSON_DUMMY_NO_SUBDIR);
        let json_dummy_dir_bad_json = Path::new(JSON_DUMMY_DIR_BAD_JSON);
        assert!(json_dummy.exists());
        assert!(json_dummy.is_file());
        assert!(json_dummy_deserialize.exists());
        assert!(json_dummy_deserialize.is_file());
        assert!(json_dummy_dir.exists());
        assert!(json_dummy_dir.is_dir());
        assert!(json_dummy_no_subdir.exists());
        assert!(json_dummy_no_subdir.is_dir());
        assert!(json_dummy_dir_bad_json.exists());
        assert!(json_dummy_dir_bad_json.is_dir());
    }

    #[test]
    fn ensure_plant_pahts_exist() {
        let plant_path = Path::new(DUMMY_PLANT_PATH);
        let species_path = Path::new(DUMMY_SPECIES_PATH);
        let logs_path = Path::new(DUMMY_LOGS_PATH);
        assert!(plant_path.exists());
        assert!(plant_path.is_dir());
        assert!(species_path.exists());
        assert!(species_path.is_dir());
        assert!(logs_path.exists());
        assert!(logs_path.is_dir());
    }
}

#[cfg(test)]
mod file_backend_tests {
    use super::{
        test_common::{
            dummy_graveyard1, dummy_graveyard2, dummy_location1, dummy_location2, dummy_location3,
            dummy_plant1, dummy_plant2, dummy_species, ACTIVITIES_DUMMY, DUMMY_LOGS_PATH,
            DUMMY_PLANT_PATH, DUMMY_SPECIES_PATH, GRAVEYARD_DUMMY, GROWTH_DUMMY, LOCATIONS_DUMMY,
        },
        FileDB,
    };
    use crate::database_manager::DatabaseManager;
    use plants::named::Named;
    use std::path::PathBuf;

    fn dummy_db() -> FileDB {
        FileDB {
            plants_dir: PathBuf::from(DUMMY_PLANT_PATH),
            species_dir: PathBuf::from(DUMMY_SPECIES_PATH),
            location_file: PathBuf::from(LOCATIONS_DUMMY),
            logs_dir: PathBuf::from(DUMMY_LOGS_PATH),
            graveyard_csv: "Graveyard.csv".to_owned(),
            growth_csv: "Growth.csv".to_owned(),
            activities_csv: "Activities.csv".to_owned(),
            date_format: "%d.%m.%Y".to_owned(),
            plants_cache: vec![],
            species_cache: vec![],
            graveyard_cache: vec![],
            location_cache: vec![],
        }
    }

    #[test]
    fn default_backend() {
        let result = FileDB::default();

        let data_dir: PathBuf = "data".into();
        let expected = FileDB {
            plants_dir: data_dir.join("Plants"),
            species_dir: data_dir.join("Species"),
            location_file: data_dir.join("Locations.csv"),
            logs_dir: data_dir.join("Logs"),
            graveyard_csv: "Graveyard.csv".to_owned(),
            growth_csv: "Growth.csv".to_owned(),
            activities_csv: "Activities.csv".to_owned(),
            date_format: "%d.%m.%Y".to_owned(),
            plants_cache: vec![],
            graveyard_cache: vec![],
            species_cache: vec![],
            location_cache: vec![],
        };

        assert_eq!(result, expected)
    }

    #[test]
    fn activities_file_path() {
        let result = FileDB::default().get_activities_filepath();
        let expected = PathBuf::from("data/Logs/Activities.csv");
        assert_eq!(result, expected)
    }

    #[test]
    fn activities_path_dummy() {
        let result = dummy_db().get_activities_filepath();
        let expected = PathBuf::from(&ACTIVITIES_DUMMY);
        assert_eq!(result, expected)
    }

    #[test]
    fn graveyard_file_path() {
        let result = FileDB::default().get_graveyard_filepath();
        let expected = PathBuf::from("data/Logs/Graveyard.csv");
        assert_eq!(result, expected)
    }

    #[test]
    fn graveyard_path_dummy() {
        let result = dummy_db().get_graveyard_filepath();
        let expected = PathBuf::from(&GRAVEYARD_DUMMY);
        assert_eq!(result, expected)
    }

    #[test]
    fn growth_file_path() {
        let result = FileDB::default().get_growth_filepath();
        let expected = PathBuf::from("data/Logs/Growth.csv");
        assert_eq!(result, expected)
    }

    #[test]
    fn growth_path_dummy() {
        let result = dummy_db().get_growth_filepath();
        let expected = PathBuf::from(&GROWTH_DUMMY);
        assert_eq!(result, expected)
    }

    #[test]
    fn load_plants() {
        let mut db = dummy_db();
        db.load_plants().unwrap();
        db.plants_cache
            .sort_by(|plant1, plant2| plant1.info.name.cmp(&plant2.info.name));
        let mut expected = vec![dummy_plant1(), dummy_plant2()];
        expected.sort_by(|plant1, plant2| plant1.info.name.cmp(&plant2.info.name));
        assert_eq!(db.plants_cache, expected)
    }

    #[test]
    fn load_species() {
        let mut db = dummy_db();
        db.load_species().unwrap();
        db.species_cache
            .sort_by(|species1, species2| species1.name.cmp(&species2.name));
        let mut expected = vec![dummy_species()];
        expected.sort_by(|species1, species2| species1.name.cmp(&species2.name));
        assert_eq!(db.species_cache, expected)
    }

    #[test]
    fn load_graveyard() {
        let mut db = dummy_db();
        db.load_graveyard().unwrap();
        db.graveyard_cache
            .sort_by(|plant1, plant2| plant1.name.cmp(&plant2.name));
        let mut expected = vec![dummy_graveyard1(), dummy_graveyard2()];
        expected.sort_by(|plant1, plant2| plant1.name.cmp(&plant2.name));
        assert_eq!(db.graveyard_cache, expected)
    }

    #[test]
    fn load_locations() {
        let mut db = dummy_db();
        db.load_locations().unwrap();
        db.location_cache
            .sort_by(|loc1, loc2| loc1.get_name().cmp(&loc2.get_name()));
        let mut expected = vec![dummy_location1(), dummy_location2(), dummy_location3()];
        expected.sort_by(|loc1, loc2| loc1.get_name().cmp(&loc2.get_name()));
        assert_eq!(db.location_cache, expected)
    }

    #[test]
    fn db_man_get_all_plants() {
        let mut db = dummy_db();
        let mut result = db.get_all_plants().unwrap();
        result.sort_by(|plant1, plant2| plant1.info.name.cmp(&plant2.info.name));
        let mut expected = vec![dummy_plant1(), dummy_plant2()];
        expected.sort_by(|plant1, plant2| plant1.info.name.cmp(&plant2.info.name));
        assert_eq!(result, expected)
    }

    #[test]
    fn db_man_get_num_plants() {
        let mut db = dummy_db();
        let result = db.get_num_plants().unwrap();
        let expected = 2;
        assert_eq!(result, expected)
    }

    #[test]
    fn db_man_get_plant() {
        let mut db = dummy_db();
        let result = db.get_plant("Dummy1").unwrap();
        let expected = dummy_plant1();
        assert_eq!(result, expected)
    }

    #[test]
    fn db_man_get_all_species() {
        let mut db = dummy_db();
        let mut result = db.get_all_species().unwrap();
        result.sort_by(|species1, species2| species1.name.cmp(&species2.name));
        let mut expected = vec![dummy_species()];
        expected.sort_by(|species1, species2| species1.name.cmp(&species2.name));
        assert_eq!(result, expected)
    }

    #[test]
    fn db_man_get_species() {
        let mut db = dummy_db();
        let result = db.get_species("test species").unwrap();
        let expected = dummy_species();
        assert_eq!(result, expected)
    }

    #[test]
    fn db_man_get_graveyard() {
        let mut db = dummy_db();
        let mut result = db.get_graveyard().unwrap();
        result.sort_by(|plant1, plant2| plant1.name.cmp(&plant2.name));
        let mut expected = vec![dummy_graveyard1(), dummy_graveyard2()];
        expected.sort_by(|plant1, plant2| plant1.name.cmp(&plant2.name));
        assert_eq!(result, expected)
    }

    #[test]
    fn db_man_get_plants_species() {
        let mut db = dummy_db();
        let result = db.get_plants_species("test species").unwrap();
        let expected = vec![dummy_plant1()];
        assert_eq!(result, expected)
    }

    #[test]
    fn db_man_get_locations() {
        let mut db = dummy_db();
        let mut result = db.get_locations().unwrap();
        result.sort_by(|loc1, loc2| loc1.get_name().cmp(&loc2.get_name()));
        let mut expected = vec![dummy_location1(), dummy_location2(), dummy_location3()];
        expected.sort_by(|loc1, loc2| loc1.get_name().cmp(&loc2.get_name()));
        assert_eq!(result, expected)
    }

    #[test]
    fn db_man_get_location() {
        let mut db = dummy_db();
        let expected = dummy_location1();
        let result = db.get_location(&expected.get_name()).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn db_man_plant_exists() {
        let mut db = dummy_db();
        let result = db.plant_exists("Dummy1").unwrap();
        assert!(result)
    }

    #[test]
    fn db_man_plant_not_exists() {
        let mut db = dummy_db();
        let result = db.plant_exists("not a plant").unwrap();
        assert!(!result)
    }

    #[test]
    fn db_man_species_exists() {
        let mut db = dummy_db();
        let result = db.species_exists("test species").unwrap();
        assert!(result)
    }

    #[test]
    fn db_man_species_not_exists() {
        let mut db = dummy_db();
        let result = db.species_exists("not a species").unwrap();
        assert!(!result)
    }

    #[test]
    fn db_man_get_plants_by_location() {
        let mut db = dummy_db();
        let result = db.get_plants_by_location("test location").unwrap();
        let expected = vec![dummy_plant1()];
        assert_eq!(result, expected)
    }

    #[test]
    fn db_man_write_growth() {}
    #[test]
    fn db_man_write_growths() {}

    #[test]
    fn db_man_write_species() {}
    #[test]
    fn db_man_write_plant() {}

    #[test]
    fn db_man_kill_plant() {}
}
