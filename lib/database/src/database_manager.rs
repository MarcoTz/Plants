use super::errors::Error;
use plants::{graveyard::GraveyardPlant, plant::Plant, species::Species};

pub trait DatabaseManager {
    fn get_all_plants(&mut self) -> Result<Vec<Plant>, Error>;
    fn get_num_plants(&mut self) -> Result<i32, Error>;
    fn get_all_species(&mut self) -> Result<Vec<Species>, Error>;
    fn get_graveyard(&mut self) -> Result<Vec<GraveyardPlant>, Error>;
}
