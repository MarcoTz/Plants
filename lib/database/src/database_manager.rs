use super::errors::Error;
use plants::plant::Plant;

pub trait DatabaseManager {
    fn get_all_plants(&self) -> Result<Vec<Plant>, Error>;
}
