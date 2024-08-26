use super::errors::Error;
use database::file_backend::{load_json::load_dir, write_json::write_species};
use plants::species::Species;
use serde::Deserialize;
use std::{io, path::PathBuf, str::FromStr};

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum FloatOrIntOrString {
    Int(i32),
    Float(f32),
    Str(String),
}

impl TryInto<i32> for FloatOrIntOrString {
    type Error = Error;
    fn try_into(self) -> Result<i32, Self::Error> {
        let new_int = match self {
            FloatOrIntOrString::Int(i) => Ok(i),
            FloatOrIntOrString::Str(st) => st.parse::<i32>(),
            FloatOrIntOrString::Float(f) => Ok(f as i32),
        }
        .map_err(|_| Error::ParseError("Int".to_owned()))?;
        Ok(new_int)
    }
}
impl TryInto<f32> for FloatOrIntOrString {
    type Error = Error;
    fn try_into(self) -> Result<f32, Self::Error> {
        let new_fl = match self {
            FloatOrIntOrString::Int(i) => Ok(i as f32),
            FloatOrIntOrString::Str(st) => st.parse::<f32>(),
            FloatOrIntOrString::Float(f) => Ok(f),
        }
        .map_err(|_| Error::ParseError("Float".to_owned()))?;
        Ok(new_fl)
    }
}

fn option_try<U, T: TryInto<U>>(opt: Option<T>) -> Result<Option<U>, T::Error> {
    match opt {
        None => Ok(None),
        Some(m_u) => {
            let u = m_u.try_into()?;
            Ok(Some(u))
        }
    }
}
#[derive(Deserialize, Clone)]
pub struct SpeciesJSON {
    name: String,
    scientific_name: String,
    species_type: String,
    sunlight_requirements: String,
    temperature_min: FloatOrIntOrString,
    temperature_max: FloatOrIntOrString,
    optimal_temperature_min: FloatOrIntOrString,
    optimal_temperature_max: FloatOrIntOrString,
    plant_distance_cm: Option<FloatOrIntOrString>,
    ph_min: FloatOrIntOrString,
    ph_max: FloatOrIntOrString,
    avg_watering_days: Option<FloatOrIntOrString>,
    watering_notes: Vec<String>,
    avg_fertilizing_days: Option<FloatOrIntOrString>,
    fertilizing_notes: Vec<String>,
    pruning_notes: Vec<String>,
    companions: Vec<String>,
    additional_notes: Vec<String>,
}

impl TryInto<Species> for SpeciesJSON {
    type Error = Error;

    fn try_into(self) -> Result<Species, Self::Error> {
        log::info!("Loading species {} from JSON", self.name);
        let new_temp_min = self.temperature_min.try_into()?;
        let new_temp_max = self.temperature_max.try_into()?;
        let new_opt_min = self.optimal_temperature_min.try_into()?;
        let new_opt_max = self.optimal_temperature_max.try_into()?;
        let new_ph_min = self.ph_min.try_into()?;
        let new_ph_max = self.ph_max.try_into()?;
        let new_dist = option_try(self.plant_distance_cm)?;
        let new_avg_water = option_try(self.avg_watering_days)?;
        let new_avg_fertilizing = option_try(self.avg_fertilizing_days)?;
        let new_sunlight = FromStr::from_str(&self.sunlight_requirements)
            .map_err(|_| Error::ParseError("Sunlight".to_owned()))?;
        Ok(Species {
            name: self.name,
            scientific_name: self.scientific_name,
            genus: "".to_owned(),
            family: "".to_owned(),
            sunlight: new_sunlight,
            temp_min: new_temp_min,
            temp_max: new_temp_max,
            opt_temp_min: new_opt_min,
            opt_temp_max: new_opt_max,
            ph_min: new_ph_min,
            ph_max: new_ph_max,
            planting_distance: new_dist,
            watering_notes: self.watering_notes,
            avg_watering_days: new_avg_water,
            fertilizing_notes: self.fertilizing_notes,
            avg_fertilizing_days: new_avg_fertilizing,
            pruning_notes: self.pruning_notes,
            companions: self.companions,
            additional_notes: self.additional_notes,
        })
    }
}

fn load_old_species(species_dir_old: &PathBuf) -> Result<Vec<SpeciesJSON>, Error> {
    let species_jsons = load_dir(species_dir_old)?;
    Ok(species_jsons)
}

fn convert_species(species: Vec<SpeciesJSON>, interactive: bool) -> Result<Vec<Species>, Error> {
    let mut new_species = vec![];
    for old_species in species.into_iter() {
        let species_ty = old_species.species_type.clone();
        let mut new_sp: Species = old_species.try_into()?;
        if interactive {
            let stdin = io::stdin();
            let mut genus = String::new();
            println!(
                "Please enter genus for {}, (type is {})",
                new_sp.name, species_ty
            );
            stdin
                .read_line(&mut genus)
                .map_err(|_| Error::InputErr("species genus".to_owned()))?;
            new_sp.genus = genus.to_owned();

            println!(
                "Please enter family for {}, (type is {})",
                new_sp.name, species_ty
            );
            let mut family = String::new();
            stdin
                .read_line(&mut family)
                .map_err(|_| Error::InputErr("species family".to_owned()))?;
            new_sp.family = family.to_owned();
        }
        new_species.push(new_sp);
    }
    Ok(new_species)
}

fn save_new_species(species: Vec<Species>, species_dir_new: &PathBuf) -> Result<(), Error> {
    write_species(species, species_dir_new)?;
    Ok(())
}

pub fn port_species(
    species_dir_old: &PathBuf,
    interactive: bool,
    species_dir_new: &PathBuf,
) -> Result<(), Error> {
    let old_species = load_old_species(species_dir_old)?;
    let new_species = convert_species(old_species, interactive)?;
    save_new_species(new_species, species_dir_new)
}
