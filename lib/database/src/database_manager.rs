use plants::{
    graveyard::GraveyardPlant,
    growth_item::GrowthItem,
    location::Location,
    log_item::LogItem,
    plant::{Plant, PlantInfo},
    species::Species,
};
use std::error::Error;

pub trait DatabaseManager {
    fn get_all_plants(&mut self) -> Result<Vec<Plant>, Box<dyn Error>>;
    fn get_plants_by_location(&mut self, location: &str) -> Result<Vec<Plant>, Box<dyn Error>>;
    fn get_num_plants(&mut self) -> Result<i32, Box<dyn Error>>;
    fn get_plant(&mut self, plant_name: &str) -> Result<Plant, Box<dyn Error>>;

    fn get_all_species(&mut self) -> Result<Vec<Species>, Box<dyn Error>>;
    fn get_species(&mut self, species_name: &str) -> Result<Species, Box<dyn Error>>;
    fn get_plants_species(&mut self, species_name: &str) -> Result<Vec<Plant>, Box<dyn Error>>;
    fn get_graveyard(&mut self) -> Result<Vec<GraveyardPlant>, Box<dyn Error>>;

    fn get_locations(&mut self) -> Result<Vec<Location>, Box<dyn Error>>;
    fn get_location(&mut self, location_name: &str) -> Result<Location, Box<dyn Error>>;

    fn plant_exists(&mut self, plant_name: &str) -> Result<bool, Box<dyn Error>>;
    fn species_exists(&mut self, species_name: &str) -> Result<bool, Box<dyn Error>>;

    fn write_logs(&mut self, logs: Vec<LogItem>) -> Result<(), Box<dyn Error>>;
    fn write_log(&mut self, log: LogItem) -> Result<(), Box<dyn Error>> {
        self.write_logs(vec![log])
    }

    fn write_growths(&mut self, growth: Vec<GrowthItem>) -> Result<(), Box<dyn Error>>;
    fn write_growth(&mut self, growth: GrowthItem) -> Result<(), Box<dyn Error>> {
        self.write_growths(vec![growth])
    }

    fn write_plant(&mut self, plant: PlantInfo) -> Result<(), Box<dyn Error>>;
    fn write_species(&mut self, species: Species) -> Result<(), Box<dyn Error>>;

    fn kill_plant(&mut self, plant: GraveyardPlant) -> Result<(), Box<dyn Error>>;
}
