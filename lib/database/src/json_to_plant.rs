use super::errors::{ConversionError, ConversionType, DBError};
use chrono::NaiveDate;
use plants::plant::Plant;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    Str(String),
}

impl TryInto<bool> for BoolOrString {
    type Error = DBError;
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

impl TryInto<Plant> for PlantJSON {
    type Error = DBError;
    fn try_into(self) -> Result<Plant, Self::Error> {
        let new_obtained = NaiveDate::parse_from_str(&self.obtained, "%d.%m.%Y")?;
        let new_autowater = self.auto_watering.try_into()?;
        Ok(Plant {
            name: self.plant_name,
            species_name: self.species_name,
            location: self.current_location,
            origin: self.origin,
            obtained: new_obtained.into(),
            auto_water: new_autowater,
            notes: self.plant_notes,
            images: vec![],
            activities: vec![],
            growth: vec![],
        })
    }
}
