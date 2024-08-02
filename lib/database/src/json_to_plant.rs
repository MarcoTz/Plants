use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    Str(String),
}
#[derive(Deserialize, Debug)]
pub struct PlantJSONOld {
    auto_watering: BoolOrString,
    current_location: String,
    obtained: String,
    origin: String,
    plant_health: String,
    plant_name: String,
    plant_notes: Vec<String>,
    species_name: String,
}

pub struct PlantJSON {
    name: String,
    species: String,
    health: i32,
    auto_watering: bool,
    current_location: String,
    obtained: String,
    notes: Vec<String>,
}
