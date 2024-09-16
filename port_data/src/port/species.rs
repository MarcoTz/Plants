use super::Port;
use crate::errors::Error;
use database::file_backend::{load_json::load_json, write_json::write_species};
use plants::species::Species;
use serde::Deserialize;
use std::{fs::read_dir, io, path::PathBuf, str::FromStr};

#[derive(Deserialize, Clone, Debug, PartialEq)]
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
        .map_err(|_| Error::Parse("Int".to_owned()))?;
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
        .map_err(|_| Error::Parse("Float".to_owned()))?;
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
#[derive(Deserialize, Clone, Debug, PartialEq)]
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
            .map_err(|_| Error::Parse("Sunlight".to_owned()))?;
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

impl Port<Vec<Species>> for Vec<SpeciesJSON> {
    type LoadArgs = PathBuf;
    type SaveArgs = PathBuf;
    type ConvertArgs = bool;

    fn load_old(species_dir_old: &Self::LoadArgs) -> Result<Vec<SpeciesJSON>, Error> {
        log::info!("Loading old species");
        let mut species_jsons = vec![];
        let dir_contents = read_dir(species_dir_old)?;
        for species_file in dir_contents {
            let file = species_file?;
            let species_json: SpeciesJSON = load_json(&file.path())?;
            species_jsons.push(species_json);
        }
        Ok(species_jsons)
    }

    fn convert(self, interactive: &Self::ConvertArgs) -> Result<Vec<Species>, Error> {
        log::info!("Converting Species");
        let mut new_species = vec![];
        for old_species in self.into_iter() {
            let species_ty = old_species.species_type.clone();
            let mut new_sp: Species = old_species.try_into()?;
            if interactive.to_owned() {
                let stdin = io::stdin();
                let mut genus = String::new();
                println!(
                    "Please enter genus for {}, (type is {})",
                    new_sp.name, species_ty
                );
                stdin
                    .read_line(&mut genus)
                    .map_err(|_| Error::Input("species genus".to_owned()))?;
                genus.clone_into(&mut new_sp.genus);

                println!(
                    "Please enter family for {}, (type is {})",
                    new_sp.name, species_ty
                );
                let mut family = String::new();
                stdin
                    .read_line(&mut family)
                    .map_err(|_| Error::Input("species family".to_owned()))?;
                family.clone_into(&mut new_sp.family);
            }
            new_species.push(new_sp);
        }
        Ok(new_species)
    }

    fn save_new(species: Vec<Species>, species_dir_new: &PathBuf) -> Result<(), Error> {
        log::info!("Saving new Species");
        write_species(species, species_dir_new)?;
        Ok(())
    }
}

#[cfg(test)]
mod species_test {
    use super::{option_try, Error, FloatOrIntOrString, Port, SpeciesJSON};
    use crate::port::test_common::{BASE_DIR, SPECIES_DIR_IN, SPECIES_DIR_OUT};
    use database::file_backend::load_json::load_dir;
    use plants::species::{Species, SunlightRequirement};
    use std::path::PathBuf;

    fn example_species_json1() -> SpeciesJSON {
        SpeciesJSON {
            name: "Species1".to_owned(),
            scientific_name: "scientific name".to_owned(),
            species_type: "will get removed".to_owned(),
            sunlight_requirements: "direct".to_owned(),
            temperature_min: FloatOrIntOrString::Float(1.0),
            temperature_max: FloatOrIntOrString::Str("1.0".to_owned()),
            optimal_temperature_min: FloatOrIntOrString::Str("1.0".to_owned()),
            optimal_temperature_max: FloatOrIntOrString::Float(1.0),
            plant_distance_cm: None,
            ph_min: FloatOrIntOrString::Float(1.0),
            ph_max: FloatOrIntOrString::Str("1.0".to_owned()),
            avg_watering_days: Some(FloatOrIntOrString::Int(1)),
            watering_notes: vec![],
            avg_fertilizing_days: Some(FloatOrIntOrString::Str("1".to_owned())),
            fertilizing_notes: vec![],
            pruning_notes: vec![],
            companions: vec![],
            additional_notes: vec![],
        }
    }

    fn example_species1() -> Species {
        Species {
            name: "Species1".to_owned(),
            scientific_name: "scientific name".to_owned(),
            genus: "".to_owned(),
            family: "".to_owned(),
            sunlight: SunlightRequirement::Direct,
            temp_min: 1.0,
            temp_max: 1.0,
            opt_temp_min: 1.0,
            opt_temp_max: 1.0,
            planting_distance: None,
            ph_min: 1.0,
            ph_max: 1.0,
            avg_watering_days: Some(1),
            watering_notes: vec![],
            avg_fertilizing_days: Some(1),
            fertilizing_notes: vec![],
            pruning_notes: vec![],
            companions: vec![],
            additional_notes: vec![],
        }
    }

    fn example_species_json2() -> SpeciesJSON {
        SpeciesJSON {
            name: "Species2".to_owned(),
            scientific_name: "scientific name".to_owned(),
            species_type: "will get removed".to_owned(),
            sunlight_requirements: "shade".to_owned(),
            temperature_min: FloatOrIntOrString::Str("1.0".to_owned()),
            temperature_max: FloatOrIntOrString::Str("1.5".to_owned()),
            optimal_temperature_min: FloatOrIntOrString::Str("1.0".to_owned()),
            optimal_temperature_max: FloatOrIntOrString::Float(1.0),
            plant_distance_cm: None,
            ph_min: FloatOrIntOrString::Float(1.0),
            ph_max: FloatOrIntOrString::Str("1.0".to_owned()),
            avg_watering_days: None,
            watering_notes: vec![],
            avg_fertilizing_days: None,
            fertilizing_notes: vec![],
            pruning_notes: vec![],
            companions: vec![],
            additional_notes: vec![],
        }
    }

    fn example_species2() -> Species {
        Species {
            name: "Species2".to_owned(),
            scientific_name: "scientific name".to_owned(),
            genus: "".to_owned(),
            family: "".to_owned(),
            sunlight: SunlightRequirement::Shade,
            temp_min: 1.0,
            temp_max: 1.5,
            opt_temp_min: 1.0,
            opt_temp_max: 1.0,
            planting_distance: None,
            ph_min: 1.0,
            ph_max: 1.0,
            avg_watering_days: None,
            watering_notes: vec![],
            avg_fertilizing_days: None,
            fertilizing_notes: vec![],
            pruning_notes: vec![],
            companions: vec![],
            additional_notes: vec![],
        }
    }

    #[test]
    fn into_int() {
        let result: i32 = FloatOrIntOrString::Int(1).try_into().unwrap();
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn into_int_err() {
        let result: Result<i32, Error> =
            FloatOrIntOrString::Str("Not a number".to_owned()).try_into();
        assert!(result.is_err())
    }

    #[test]
    fn into_float() {
        let result: f32 = FloatOrIntOrString::Float(1.0).try_into().unwrap();
        let expected = 1.0;
        assert_eq!(result, expected)
    }

    #[test]
    fn into_float_err() {
        let result: Result<f32, Error> =
            FloatOrIntOrString::Str("Not a number".to_owned()).try_into();
        assert!(result.is_err())
    }

    #[test]
    fn into_option() {
        let result: Option<i32> = option_try(Some(FloatOrIntOrString::Int(1))).unwrap();
        let expected = Some(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn into_option_err() {
        let result: Result<Option<i32>, Error> =
            option_try(Some(FloatOrIntOrString::Str("Not a number".to_owned())));
        assert!(result.is_err())
    }

    #[test]
    fn into_species() {
        let result: Species = example_species_json1().try_into().unwrap();
        let expected = example_species1();
        assert_eq!(result, expected)
    }

    #[test]
    fn load_old() {
        let species_dir = PathBuf::from(BASE_DIR).join(SPECIES_DIR_IN);
        let result = <Vec<SpeciesJSON> as Port<Vec<Species>>>::load_old(&species_dir).unwrap();
        let expected = vec![example_species_json1(), example_species_json2()];
        assert_eq!(result, expected)
    }

    #[test]
    fn convert() {
        let result = vec![example_species_json1(), example_species_json2()]
            .convert(&false)
            .unwrap();
        let expected = vec![example_species1(), example_species2()];
        assert_eq!(result, expected)
    }

    #[test]
    fn save_new() {
        let species_dir = PathBuf::from(BASE_DIR).join(SPECIES_DIR_OUT);
        if !species_dir.exists() {
            std::fs::create_dir_all(species_dir.clone()).unwrap();
        }
        assert!(species_dir.exists());

        <Vec<SpeciesJSON> as Port<Vec<Species>>>::save_new(
            vec![example_species1(), example_species2()],
            &species_dir,
        )
        .unwrap();

        let dir1 = species_dir.join("Species1");
        let dir2 = species_dir.join("Species2");
        let file1 = dir1.join("Species1.json");
        let file2 = dir2.join("Species2.json");
        assert!(file1.exists());
        assert!(file2.exists());

        let result: Vec<Species> = load_dir(&species_dir).unwrap();
        let expected = vec![example_species2(), example_species1()];
        assert_eq!(result, expected);

        std::fs::remove_dir_all(dir1.clone()).unwrap();
        std::fs::remove_dir_all(dir2.clone()).unwrap();
        assert!(!dir1.exists());
        assert!(!dir2.exists());
    }
}
