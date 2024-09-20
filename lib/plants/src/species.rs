use super::{
    errors::Error,
    named::Named,
    plant::{Plant, PlantSpecies},
};
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Serialize, PartialEq, Deserialize, Clone, Debug)]
pub enum SunlightRequirement {
    Direct,
    Indirect,
    Shade,
}

#[derive(Serialize, PartialEq, Deserialize, Clone, Debug)]
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
        log::info!(
            "Getting time delta between {} for {}",
            activity_name,
            self.name
        );
        match activity_name.to_lowercase().trim() {
            "watering" => self.avg_watering_days.map(|x| TimeDelta::days(x as i64)),
            "fertilizing" => self.avg_fertilizing_days.map(|x| TimeDelta::days(x as i64)),
            _ => None,
        }
    }

    pub fn get_url(&self, base: &str) -> String {
        let prefix = if base.is_empty() {
            "".to_owned()
        } else {
            base.to_owned() + "/"
        };
        prefix + &self.get_name().replace(' ', "") + ".html"
    }

    pub fn get_plants(&self, plants: &[Plant]) -> Vec<Plant> {
        let mut species_plants = vec![];
        for plant in plants.iter() {
            match &plant.info.species {
                PlantSpecies::Other(_) => (),
                PlantSpecies::Species(species) => {
                    if species.name == self.name {
                        species_plants.push(plant.clone())
                    }
                }
            }
        }
        species_plants
    }
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

#[cfg(test)]
mod species_tests {
    use super::SunlightRequirement;
    use crate::test_common::{empty_plant, example_plant, example_plant2, example_species};
    use chrono::TimeDelta;
    use std::str::FromStr;

    #[test]
    fn sunlight_direct() {
        let result = SunlightRequirement::from_str("direct").unwrap();
        let expected = SunlightRequirement::Direct;
        assert_eq!(result, expected)
    }

    #[test]
    fn sunlight_indirect() {
        let result = SunlightRequirement::from_str("indirect").unwrap();
        let expected = SunlightRequirement::Indirect;
        assert_eq!(result, expected)
    }

    #[test]
    fn sunlight_shade() {
        let result = SunlightRequirement::from_str("shade").unwrap();
        let expected = SunlightRequirement::Shade;
        assert_eq!(result, expected)
    }

    #[test]
    fn display_direct() {
        let result = format!("{}", SunlightRequirement::Direct);
        let expected = "Direct".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_indirect() {
        let result = format!("{}", SunlightRequirement::Indirect);
        let expected = "Indirect".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_shade() {
        let result = format!("{}", SunlightRequirement::Shade);
        let expected = "Shade".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn watering_delta() {
        let result = example_species().get_activity_delta("watering");
        let expected = Some(TimeDelta::days(7));
        assert_eq!(result, expected)
    }

    #[test]
    fn fertilizing_delta() {
        let result = example_species().get_activity_delta("fertilizing");
        let expected = Some(TimeDelta::days(14));
        assert_eq!(result, expected)
    }

    #[test]
    fn delta_none() {
        let result = example_species().get_activity_delta("other");
        let expected = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn species_plants() {
        let result = example_species()
            .get_plants(vec![example_plant(), example_plant2(), empty_plant()].as_slice());
        let expected = vec![example_plant(), example_plant2()];
        assert_eq!(result, expected)
    }

    #[test]
    fn species_url() {
        let result = example_species().get_url("species");
        let expected = "species/Testspecies.html";
        assert_eq!(result, expected)
    }
}
