use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FloatOrIntOrString {
    Int(i32),
    Float(f32),
    Str(String),
}

#[derive(Deserialize)]
pub struct SpeciesJSONOld {
    name: String,
    scientific_name: String,
    species_type: String,
    sunlight_requirements: String,
    temperature_min: FloatOrIntOrString,
    temperature_max: FloatOrIntOrString,
    optimal_temperature_min: FloatOrIntOrString,
    optimal_temperature_max: FloatOrIntOrString,
    plant_distance_cm: FloatOrIntOrString,
    ph_min: FloatOrIntOrString,
    ph_max: FloatOrIntOrString,
    avg_watering_days: FloatOrIntOrString,
    watering_notes: Vec<String>,
    avg_fertilizing_days: FloatOrIntOrString,
    fertilizing_notes: Vec<String>,
    pruning_notes: Vec<String>,
    companions: Vec<String>,
    additional_notes: Vec<String>,
}

pub struct SpeciesJSON {
    name: String,
    scientific_name: String,
    genus: String,
    family: String,
    sunlight: String,
    temp_min: f32,
    temp_max: f32,
    opt_temp_min: f32,
    opt_temp_max: f32,
    planting_distance: f32,
    ph_min: f32,
    ph_max: f32,
    watering_notes: Vec<String>,
    avg_watering_days: i32,
    fertilizing_notes: Vec<String>,
    avg_fertilizing_days: i32,
    pruning_notes: Vec<String>,
    companions: Vec<String>,
    additional_notes: Vec<String>,
}
