use super::errors::Error;
use plants::{
    graveyard::GraveyardPlant, growth_item::GrowthItem, log_item::LogItem, plant::Plant,
    species::Species,
};
use serde::{Deserialize, Serialize};

pub trait DatabaseManager {
    fn get_all_plants(&mut self) -> Result<Vec<Plant>, Error>;
    fn get_plants_by_location(&mut self, location: String) -> Result<Vec<Plant>, Error>;
    fn get_num_plants(&mut self) -> Result<i32, Error>;
    fn get_all_species(&mut self) -> Result<Vec<Species>, Error>;
    fn get_species(&mut self, name: &str) -> Result<Species, Error>;
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

    fn write_plant(&mut self, plant: PlantJSON) -> Result<(), Error>;
    fn write_species(&mut self, species: Species) -> Result<(), Error>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlantJSON {
    pub auto_watering: BoolOrString,
    pub current_location: String,
    pub obtained: String,
    pub origin: String,
    pub plant_health: String,
    pub plant_name: String,
    pub plant_notes: Vec<String>,
    pub species_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    Str(String),
}

impl From<bool> for BoolOrString {
    fn from(b: bool) -> BoolOrString {
        BoolOrString::Bool(b)
    }
}
impl TryInto<bool> for BoolOrString {
    type Error = crate::file_backend::errors::Error;
    fn try_into(self) -> Result<bool, Self::Error> {
        let new_b = match self {
            BoolOrString::Bool(b) => Ok(b),
            BoolOrString::Str(st) => {
                if st.as_str() == "y" {
                    Ok(true)
                } else if st.as_str() == "n" {
                    Ok(false)
                } else {
                    st.to_lowercase().trim().parse::<bool>()
                }
            }
        }?;
        Ok(new_b)
    }
}
