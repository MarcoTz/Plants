use super::{plants::PlantJSON, Port};
use crate::errors::Error;
use chrono::NaiveDate;
use database::file_backend::{load_csv::load_csv, write_csv::write_csv};
use plants::{growth_item::GrowthItem, serialize::date_serializer};
use serde::Deserialize;
use std::{fs::File, path::PathBuf};

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GrowthCSV {
    #[serde(with = "date_serializer")]
    date: NaiveDate,
    plant: String,
    height: f32,
    width: f32,
    note: Option<String>,
}

impl From<GrowthCSV> for GrowthItem {
    fn from(growth_csv: GrowthCSV) -> GrowthItem {
        GrowthItem {
            plant: growth_csv.plant,
            date: growth_csv.date,
            height_cm: growth_csv.height,
            width_cm: growth_csv.width,
            note: growth_csv.note,
            health: 3,
        }
    }
}
impl Port<Vec<GrowthItem>> for Vec<GrowthCSV> {
    type LoadArgs = PathBuf;
    type SaveArgs = PathBuf;
    type ConvertArgs = Vec<PlantJSON>;

    fn load_old(growth_file: &Self::LoadArgs) -> Result<Vec<GrowthCSV>, Error> {
        log::info!("Loading old growth");
        let old_growth = load_csv(growth_file)?;
        Ok(old_growth)
    }

    fn convert(self, plants: &Self::ConvertArgs) -> Result<Vec<GrowthItem>, Error> {
        log::info!("Converting Growth");
        let mut new_items = vec![];
        for old_item in self.into_iter() {
            let mut new_item: GrowthItem = old_item.into();
            let growth_plant = plants
                .iter()
                .find(|pl| pl.plant_name == new_item.plant.trim())
                .ok_or(Error::PlantNotFound(new_item.plant.clone()))?;
            let health = growth_plant.plant_health.parse::<i32>()?;
            if !(0..=5).contains(&health) {
                Err(Error::BadHealth(health))
            } else {
                Ok(())
            }?;
            new_item.health = health;
            new_items.push(new_item);
        }
        Ok(new_items)
    }

    fn save_new(new: Vec<GrowthItem>, growth_file_out: &Self::SaveArgs) -> Result<(), Error> {
        log::info!("Saving new Growth");
        if !growth_file_out.exists() {
            log::info!("Creating growth log");
            File::create(growth_file_out)?;
        }
        write_csv(new, growth_file_out, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod growth_tests {
    use super::{GrowthCSV, GrowthItem, Port};
    use crate::port::test_common::{
        example_date1, example_date2, example_plant_json1, example_plant_json2,
        example_plant_json3, BASE_DIR, GROWTH_FILE_IN, GROWTH_FILE_OUT,
    };
    use database::file_backend::load_csv::load_csv;
    use std::path::PathBuf;

    fn example_growth_csv1() -> GrowthCSV {
        GrowthCSV {
            date: example_date1(),
            plant: "Plant1".to_owned(),
            height: 10.0,
            width: 10.0,
            note: None,
        }
    }

    fn example_growth1() -> GrowthItem {
        GrowthItem {
            date: example_date1(),
            plant: "Plant1".to_owned(),
            height_cm: 10.0,
            width_cm: 10.0,
            note: None,
            health: 3,
        }
    }

    fn example_growth_csv2() -> GrowthCSV {
        GrowthCSV {
            date: example_date1(),
            plant: "Plant2".to_owned(),
            height: 10.0,
            width: 10.0,
            note: None,
        }
    }

    fn example_growth2() -> GrowthItem {
        GrowthItem {
            date: example_date1(),
            plant: "Plant2".to_owned(),
            height_cm: 10.0,
            width_cm: 10.0,
            note: None,
            health: 4,
        }
    }

    fn example_growth_csv3() -> GrowthCSV {
        GrowthCSV {
            date: example_date2(),
            plant: "Plant3".to_owned(),
            height: 5.0,
            width: 5.0,
            note: None,
        }
    }

    fn example_growth3() -> GrowthItem {
        GrowthItem {
            date: example_date2(),
            plant: "Plant3".to_owned(),
            height_cm: 5.0,
            width_cm: 5.0,
            note: None,
            health: 3,
        }
    }

    #[test]
    fn csv_to_growth() {
        let result = <GrowthCSV as Into<GrowthItem>>::into(example_growth_csv1());
        let expected = example_growth1();
        assert_eq!(result, expected)
    }

    #[test]
    fn load_old() {
        let growth_file = PathBuf::from(BASE_DIR).join(GROWTH_FILE_IN);
        let result = <Vec<GrowthCSV> as Port<Vec<GrowthItem>>>::load_old(&growth_file).unwrap();
        let expected = vec![
            example_growth_csv1(),
            example_growth_csv2(),
            example_growth_csv3(),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn convert() {
        let result = vec![
            example_growth_csv1(),
            example_growth_csv2(),
            example_growth_csv3(),
        ]
        .convert(&vec![
            example_plant_json1(),
            example_plant_json2(),
            example_plant_json3(),
        ])
        .unwrap();
        let expected = vec![example_growth1(), example_growth2(), example_growth3()];
        assert_eq!(result, expected)
    }

    #[test]
    fn save_new() {
        let growth_file = PathBuf::from(BASE_DIR).join(GROWTH_FILE_OUT);
        if growth_file.exists() {
            std::fs::remove_file(growth_file.clone()).unwrap();
        }
        assert!(!growth_file.exists());

        <Vec<GrowthCSV> as Port<Vec<GrowthItem>>>::save_new(
            vec![example_growth1(), example_growth2(), example_growth3()],
            &growth_file,
        )
        .unwrap();

        assert!(growth_file.exists());
        let result: Vec<GrowthItem> = load_csv(&growth_file).unwrap();
        let expected = vec![example_growth1(), example_growth2(), example_growth3()];
        assert_eq!(result, expected);

        std::fs::remove_file(growth_file.clone()).unwrap();
        assert!(!growth_file.exists())
    }
}
