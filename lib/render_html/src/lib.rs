pub mod errors;
pub mod renderer;
pub mod write_html;

#[cfg(test)]
pub mod test_common {
    use super::renderer::Renderer;
    use chrono::NaiveDate;
    use database::database_manager::DatabaseManager;
    use plants::{
        graveyard::GraveyardPlant,
        growth_item::GrowthItem,
        location::Location,
        log_item::LogItem,
        plant::{Plant, PlantImage, PlantInfo, PlantLocation, PlantSpecies},
        species::{Species, SunlightRequirement},
    };
    use std::{error::Error, fmt, path::PathBuf};

    pub const DATE_FORMAT: &str = "%d.%m.%Y";

    pub fn example_graveyard() -> GraveyardPlant {
        GraveyardPlant {
            name: "dead plant1".to_owned(),
            species: "species".to_owned(),
            planted: example_date1(),
            died: example_date1(),
            reason: "death".to_owned(),
        }
    }

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
    pub fn example_location() -> Location {
        Location {
            name: "Inside".to_owned(),
            outside: false,
        }
    }

    pub fn example_date1() -> NaiveDate {
        NaiveDate::parse_from_str("01.01.1970", "%d.%m.%Y").unwrap()
    }

    pub fn example_date2() -> NaiveDate {
        NaiveDate::parse_from_str("02.01.1970", "%d.%m.%Y").unwrap()
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

    pub fn example_plant2() -> Plant {
        let mut plant2 = example_plant();
        plant2.info.name = "Another Plant".to_owned();
        plant2.info.obtained = example_date2();
        let mut growth1 = example_growth1();
        growth1.plant = "Another Plant".to_owned();
        growth1.height_cm = 5.0;
        growth1.width_cm = 5.0;
        let mut growth2 = example_growth2();
        growth2.plant = "Another Plant".to_owned();
        growth2.height_cm = 12.0;
        growth2.width_cm = 12.0;
        plant2.growth = vec![growth1, growth2];
        plant2
    }

    pub struct DummyManager;
    #[derive(Debug)]
    pub struct DummyError;
    impl fmt::Display for DummyError {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            panic!("not implemented")
        }
    }
    impl Error for DummyError {}

    impl DatabaseManager for DummyManager {
        fn find_plant_name(&mut self, _: String) -> Result<String, Box<dyn Error>> {
            panic!("not implemented")
        }

        fn find_species_name(&mut self, _: String) -> Result<String, Box<dyn Error>> {
            panic!("not implemented")
        }

        fn get_all_plants(&mut self) -> Result<Vec<Plant>, Box<dyn Error>> {
            Ok(vec![example_plant(), example_plant2()])
        }

        fn get_plants_by_location(&mut self, _: &str) -> Result<Vec<Plant>, Box<dyn Error>> {
            panic!("not implemented")
        }

        fn get_plant(&mut self, _: &str) -> Result<Plant, Box<dyn Error>> {
            panic!("not implemented")
        }

        fn get_plants_species(&mut self, _: &str) -> Result<Vec<Plant>, Box<dyn Error>> {
            panic!("not implemented")
        }

        fn get_num_plants(&mut self) -> Result<i32, Box<dyn Error>> {
            Ok(self.get_all_plants().unwrap().len() as i32)
        }

        fn write_plants(&mut self, _: Vec<PlantInfo>) -> Result<(), Box<dyn Error>> {
            panic!("not implemented")
        }

        fn get_all_species(&mut self) -> Result<Vec<Species>, Box<dyn Error>> {
            Ok(vec![example_species()])
        }

        fn get_species(&mut self, _: &str) -> Result<Species, Box<dyn Error>> {
            panic!("not implemented")
        }

        fn write_species(&mut self, _: Species) -> Result<(), Box<dyn Error>> {
            panic!("not implemented")
        }

        fn get_graveyard(&mut self) -> Result<Vec<GraveyardPlant>, Box<dyn Error>> {
            Ok(vec![example_graveyard()])
        }

        fn kill_plant(&mut self, _: GraveyardPlant) -> Result<(), Box<dyn Error>> {
            panic!("not implemented")
        }

        fn get_locations(&mut self) -> Result<Vec<Location>, Box<dyn Error>> {
            panic!("not implemented")
        }
        fn write_location(&mut self, _: Location) -> Result<(), Box<dyn Error>> {
            panic!("not implemented")
        }

        fn get_location(&mut self, _: &str) -> Result<Location, Box<dyn Error>> {
            panic!("not implemented")
        }

        fn get_logs(&mut self) -> Result<Vec<LogItem>, Box<dyn Error>> {
            panic!("not implemented")
        }

        fn write_logs(&mut self, _: Vec<LogItem>) -> Result<(), Box<dyn Error>> {
            panic!("not implemented")
        }

        fn get_growth(&mut self) -> Result<Vec<GrowthItem>, Box<dyn Error>> {
            panic!("not implemented")
        }

        fn write_growths(&mut self, _: Vec<GrowthItem>) -> Result<(), Box<dyn Error>> {
            panic!("not implemented")
        }

        fn plant_exists(&mut self, plant_name: &str) -> Result<bool, Box<dyn Error>> {
            Ok(self.get_plant(plant_name).is_ok())
        }
        fn species_exists(&mut self, species_name: &str) -> Result<bool, Box<dyn Error>> {
            Ok(self.get_species(species_name).is_ok())
        }
    }

    pub fn example_renderer() -> Renderer<DummyManager> {
        Renderer {
            database_manager: DummyManager {},
            date_format: DATE_FORMAT.to_owned(),
        }
    }

    #[test]
    #[should_panic]
    fn display_err() {
        let _ = format!("{}", DummyError);
    }

    #[test]
    #[should_panic]
    fn by_location() {
        DummyManager {}.get_plants_by_location("").unwrap();
    }

    #[test]
    #[should_panic]
    fn plant() {
        DummyManager {}.get_plant("").unwrap();
    }

    #[test]
    #[should_panic]
    fn species_plants() {
        DummyManager {}.get_plants_species("").unwrap();
    }

    #[test]
    #[should_panic]
    fn write_plants() {
        DummyManager {}.write_plants(vec![]).unwrap();
    }

    #[test]
    #[should_panic]
    fn speices() {
        DummyManager {}.get_species("").unwrap();
    }

    #[test]
    #[should_panic]
    fn write_species() {
        DummyManager {}.write_species(example_species()).unwrap();
    }

    #[test]
    #[should_panic]
    fn kill() {
        DummyManager {}.kill_plant(example_graveyard()).unwrap();
    }

    #[test]
    #[should_panic]
    fn locations() {
        DummyManager {}.get_locations().unwrap();
    }

    #[test]
    #[should_panic]
    fn location() {
        DummyManager {}.get_location("").unwrap();
    }

    #[test]
    #[should_panic]
    fn logs() {
        DummyManager {}.get_logs().unwrap();
    }

    #[test]
    #[should_panic]
    fn write_logs() {
        DummyManager {}.write_logs(vec![]).unwrap();
    }

    #[test]
    #[should_panic]
    fn growth() {
        DummyManager {}.get_growth().unwrap();
    }

    #[test]
    #[should_panic]
    fn write_growth() {
        DummyManager {}.write_growth(example_growth1()).unwrap();
    }

    #[test]
    #[should_panic]
    fn plant_exists() {
        DummyManager {}.plant_exists("").unwrap();
    }

    #[test]
    #[should_panic]
    fn species_exists() {
        DummyManager {}.species_exists("").unwrap();
    }
}
