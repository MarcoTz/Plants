use super::errors::PlantError;
use std::str::FromStr;

pub enum SunlightRequirement {
    Direct,
    Indirect,
    Shade,
}

impl FromStr for SunlightRequirement {
    type Err = PlantError;
    fn from_str(s: &str) -> Result<SunlightRequirement, PlantError> {
        match s.trim().to_lowercase().as_str() {
            "direct" => Ok(SunlightRequirement::Direct),
            "indirect" => Ok(SunlightRequirement::Indirect),
            "shade" => Ok(SunlightRequirement::Shade),
            _ => Err(PlantError::SunlightError(s.to_owned())),
        }
    }
}
pub struct Species {
    pub name: String,
    pub scientific_name: String,
    pub genus: String,
    pub family: String,
    pub sunlight: SunlightRequirement,
    pub temp_min: f32,
    pub temp_max: f32,
    pub opt_temp_min: f32,
    pub opt_temp_max: f32,
    pub planting_distance: Option<f32>,
    pub ph_min: f32,
    pub ph_max: f32,
    pub watering_notes: Vec<String>,
    pub avg_watering_days: Option<i32>,
    pub fertilizing_notes: Vec<String>,
    pub avg_fertilizing_days: Option<i32>,
    pub pruning_notes: Vec<String>,
    pub companions: Vec<String>,
    pub additional_notes: Vec<String>,
}
