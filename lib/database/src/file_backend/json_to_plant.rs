use super::{
    csv_to_growth_item::GrowthCSV, csv_to_log_item::LogCSV, errors::Error, file_db,
    json_to_species::SpeciesJSON,
};
use chrono::NaiveDate;
use plants::{log_item::LogItem, plant::Plant};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    Str(String),
}

impl TryInto<bool> for BoolOrString {
    type Error = Error;
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

#[derive(Deserialize, Debug, Clone)]
pub struct PlantJSON {
    auto_watering: BoolOrString,
    current_location: String,
    obtained: String,
    origin: String,
    plant_health: String,
    plant_name: String,
    plant_notes: Vec<String>,
    species_name: String,
}

struct PlantInfo {
    plant: PlantJSON,
    species: SpeciesJSON,
    logs: Vec<LogCSV>,
    growth: Vec<GrowthCSV>,
}

impl TryInto<Plant> for PlantInfo {
    type Error = Error;
    fn try_into(self) -> Result<Plant, Self::Error> {
        let db_man = file_db::get_default();
        let new_obtained = NaiveDate::parse_from_str(&self.plant.obtained, &db_man.date_format)?;
        let new_autowater = self.plant.auto_watering.try_into()?;
        let species = self.species.try_into()?;
        Ok(Plant {
            name: self.plant.plant_name,
            species: Some(species),
            location: self.plant.current_location,
            origin: self.plant.origin,
            obtained: new_obtained,
            auto_water: new_autowater,
            notes: self.plant.plant_notes,
            activities: self
                .logs
                .iter()
                .cloned()
                .flat_map(|log| <LogCSV as Into<Vec<LogItem>>>::into(log))
                .collect(),
            growth: vec![],
        })
    }
}
