use super::errors::Error;
use plants::{
    graveyard::GraveyardPlant,
    growth_item::GrowthItem,
    log_item::LogItem,
    plant::{Plant, PlantInfo},
    species::Species,
};

pub trait DatabaseManager {
    fn get_all_plants(&mut self) -> Result<Vec<Plant>, Error>;
    fn get_plants_by_location(&mut self, location: String) -> Result<Vec<Plant>, Error>;
    fn get_num_plants(&mut self) -> Result<i32, Error>;
    fn get_plant(&mut self, plant_name: &str) -> Result<Plant, Error>;

    fn get_all_species(&mut self) -> Result<Vec<Species>, Error>;
    fn get_species(&mut self, species_name: &str) -> Result<Species, Error>;
    fn get_plants_species(&mut self, species_name: &str) -> Result<Vec<Plant>, Error>;
    fn get_graveyard(&mut self) -> Result<Vec<GraveyardPlant>, Error>;

    fn plant_exists(&mut self, plant_name: &str) -> Result<bool, Error>;
    fn species_exists(&mut self, species_name: &str) -> Result<bool, Error>;

    fn write_logs(&mut self, logs: Vec<LogItem>) -> Result<(), Error>;
    fn write_log(&mut self, log: LogItem) -> Result<(), Error> {
        self.write_logs(vec![log])
    }

    fn write_growths(&mut self, growth: Vec<GrowthItem>) -> Result<(), Error>;
    fn write_growth(&mut self, growth: GrowthItem) -> Result<(), Error> {
        self.write_growths(vec![growth])
    }

    fn write_plant(&mut self, plant: PlantInfo) -> Result<(), Error>;
    fn write_species(&mut self, species: Species) -> Result<(), Error>;
    fn kill_plant(&mut self, plant: GraveyardPlant) -> Result<(), Error>;
}
