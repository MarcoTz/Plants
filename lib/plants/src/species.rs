use super::{errors::Error, plant::Plant};
use chrono::TimeDelta;
use serde::Serialize;
use std::{fmt, str::FromStr};

#[derive(Serialize, Clone, Debug)]
pub enum SunlightRequirement {
    Direct,
    Indirect,
    Shade,
}

impl FromStr for SunlightRequirement {
    type Err = Error;
    fn from_str(s: &str) -> Result<SunlightRequirement, Error> {
        match s.trim().to_lowercase().as_str() {
            "direct" => Ok(SunlightRequirement::Direct),
            "indirect" => Ok(SunlightRequirement::Indirect),
            "shade" => Ok(SunlightRequirement::Shade),
            _ => Err(Error::SunlightError(s.to_owned())),
        }
    }
}

impl fmt::Display for SunlightRequirement {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SunlightRequirement::Direct => frmt.write_str("Direct"),
            SunlightRequirement::Indirect => frmt.write_str("Indirect"),
            SunlightRequirement::Shade => frmt.write_str("Shade"),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
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

impl Species {
    pub fn get_activity_delta(&self, activity_name: &str) -> Option<TimeDelta> {
        match activity_name.to_lowercase().trim() {
            "watering" => self.avg_watering_days.map(|x| TimeDelta::days(x as i64)),
            "fertilizing" => self.avg_fertilizing_days.map(|x| TimeDelta::days(x as i64)),
            _ => None,
        }
    }

    pub fn get_url(&self, base: &str) -> String {
        let mut url = base.to_owned();
        url.push('/');
        url.push_str(&self.name.replace(' ', ""));
        url.push_str(".html");
        url
    }

    pub fn get_plants(&self, plants: &[Plant]) -> Vec<Plant> {
        let mut species_plants = vec![];
        for plant in plants.iter() {
            match plant.species.as_ref() {
                None => (),
                Some(species) => {
                    if species.name == self.name {
                        species_plants.push(plant.clone())
                    }
                }
            }
        }
        species_plants
    }
}
