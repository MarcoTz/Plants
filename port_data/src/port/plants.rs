use super::Port;
use crate::errors::Error;
use chrono::NaiveDate;
use database::file_backend::{load_json::load_json, write_json::write_plants};
use plants::plant::{PlantInfo, PlantLocation, PlantSpecies};
use serde::{Deserialize, Serialize};
use std::{fs::read_dir, path::PathBuf};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlantJSON {
    pub auto_watering: BoolOrString,
    pub current_location: String,
    pub obtained: String,
    pub origin: String,
    pub plant_health: String,
    pub plant_name: String,
    pub plant_notes: Vec<String>,
    pub species_name: String,
}

#[derive(Hash, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    Str(String),
}

impl From<bool> for BoolOrString {
    fn from(b: bool) -> BoolOrString {
        BoolOrString::Bool(b)
    }
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

impl Port<Vec<PlantInfo>> for Vec<PlantJSON> {
    type LoadArgs = PathBuf;
    type SaveArgs = PathBuf;
    type ConvertArgs = String;

    fn load_old(plants_dir: &Self::LoadArgs) -> Result<Vec<PlantJSON>, Error> {
        log::info!("Loading old plant infos");
        let mut plants = vec![];
        let contents = read_dir(plants_dir)?;
        for plant_file in contents {
            let file = plant_file?;
            let plant: PlantJSON = load_json(&file.path())?;
            plants.push(plant);
        }
        Ok(plants)
    }

    fn convert(self, date_format: &Self::ConvertArgs) -> Result<Vec<PlantInfo>, Error> {
        log::info!("Converting plant infos");
        let mut new_plants = vec![];
        for old_plant in self.into_iter() {
            let obtained = NaiveDate::parse_from_str(&old_plant.obtained, date_format)?;
            let auto_water = old_plant.auto_watering.try_into()?;
            let new_plant = PlantInfo {
                name: old_plant.plant_name,
                species: PlantSpecies::Other(old_plant.species_name),
                location: PlantLocation::Other(old_plant.current_location),
                origin: old_plant.origin,
                obtained,
                auto_water,
                notes: old_plant.plant_notes,
            };
            new_plants.push(new_plant);
        }
        Ok(new_plants)
    }

    fn save_new(plants: Vec<PlantInfo>, plants_dir: &Self::SaveArgs) -> Result<(), Error> {
        log::info!("Saving new Plants");
        write_plants(plants, plants_dir)?;
        Ok(())
    }
}

#[cfg(test)]
mod plants_tests {
    use super::{PlantInfo, PlantJSON, PlantLocation, PlantSpecies, Port};
    use crate::port::test_common::{
        example_date1, example_date2, example_plant_json1, example_plant_json2,
        example_plant_json3, BASE_DIR, PLANTS_DIR_IN, PLANTS_DIR_OUT,
    };
    use database::file_backend::load_json::load_dir;
    use std::{collections::HashSet, path::PathBuf};

    fn example_info1() -> PlantInfo {
        PlantInfo {
            name: "Plant1".to_owned(),
            species: PlantSpecies::Other("Species1".to_owned()),
            location: PlantLocation::Other("Location1".to_owned()),
            origin: "test origin".to_owned(),
            obtained: example_date1(),
            auto_water: false,
            notes: vec![],
        }
    }

    fn example_info2() -> PlantInfo {
        PlantInfo {
            name: "Plant2".to_owned(),
            species: PlantSpecies::Other("Species1".to_owned()),
            location: PlantLocation::Other("Location2".to_owned()),
            origin: "test origin".to_owned(),
            obtained: example_date1(),
            auto_water: false,
            notes: vec![],
        }
    }

    fn example_info3() -> PlantInfo {
        PlantInfo {
            name: "Plant3".to_owned(),
            species: PlantSpecies::Other("Species2".to_owned()),
            location: PlantLocation::Other("Location1".to_owned()),
            origin: "test origin".to_owned(),
            obtained: example_date2(),
            auto_water: true,
            notes: vec![],
        }
    }

    #[test]
    fn load_old() {
        let plants_dir = PathBuf::from(BASE_DIR).join(PLANTS_DIR_IN);
        let result = HashSet::from_iter(
            <Vec<PlantJSON> as Port<Vec<PlantInfo>>>::load_old(&plants_dir)
                .unwrap()
                .iter()
                .cloned(),
        );
        let expected = HashSet::from([
            example_plant_json3(),
            example_plant_json2(),
            example_plant_json1(),
        ]);
        assert_eq!(result, expected)
    }

    #[test]
    fn convert() {
        let result: Vec<PlantInfo> = vec![
            example_plant_json1(),
            example_plant_json2(),
            example_plant_json3(),
        ]
        .convert(&"%d.%m.%Y".to_owned())
        .unwrap();
        let expected = vec![example_info1(), example_info2(), example_info3()];
        assert_eq!(result, expected)
    }

    #[test]
    fn save_new() {
        let plant_dir = PathBuf::from(BASE_DIR).join(PLANTS_DIR_OUT);

        let dir1 = plant_dir.join("Plant1");
        let file1 = dir1.join("Plant1.json");
        let dir2 = plant_dir.join("Plant2");
        let file2 = dir2.join("Plant2.json");
        let dir3 = plant_dir.join("Plant3");
        let file3 = dir3.join("Plant3.json");

        if file1.exists() {
            std::fs::remove_file(file1.clone()).unwrap();
        }
        if file2.exists() {
            std::fs::remove_file(file2.clone()).unwrap();
        }
        if file3.exists() {
            std::fs::remove_file(file3.clone()).unwrap();
        }

        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(!file3.exists());

        <Vec<PlantJSON> as Port<Vec<PlantInfo>>>::save_new(
            vec![example_info1(), example_info2(), example_info3()],
            &plant_dir,
        )
        .unwrap();

        assert!(file1.exists());
        assert!(file2.exists());
        assert!(file3.exists());

        let result = HashSet::from_iter(load_dir(&plant_dir).unwrap().iter().cloned());
        let expected = HashSet::from([example_info3(), example_info1(), example_info2()]);
        assert_eq!(result, expected);

        std::fs::remove_file(file1.clone()).unwrap();
        std::fs::remove_file(file2.clone()).unwrap();
        std::fs::remove_file(file3.clone()).unwrap();
        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(!file3.exists());
    }
}
