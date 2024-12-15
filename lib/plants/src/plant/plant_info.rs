use super::{PlantLocation, PlantSpecies};
use crate::serialize::{date_serializer, location_serializer, species_serializer};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub struct PlantInfo {
    pub name: String,
    #[serde(with = "species_serializer")]
    pub species: PlantSpecies,
    #[serde(with = "location_serializer")]
    pub location: PlantLocation,
    pub origin: String,
    #[serde(with = "date_serializer")]
    pub obtained: NaiveDate,
    pub auto_water: bool,
    pub notes: Vec<String>,
}
