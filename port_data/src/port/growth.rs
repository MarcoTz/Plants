use super::{plants::PlantJSON, Port};
use crate::errors::Error;
use chrono::NaiveDate;
use database::file_backend::{load_csv::load_csv, write_csv::write_csv};
use plants::{growth_item::GrowthItem, serialize::date_serializer};
use serde::Deserialize;
use std::{fs::File, path::PathBuf};

#[derive(Deserialize, Clone)]
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
        let old_growth = load_csv(growth_file)?;
        Ok(old_growth)
    }

    fn convert(self, plants: &Self::ConvertArgs) -> Result<Vec<GrowthItem>, Error> {
        let mut new_items = vec![];
        for old_item in self.into_iter() {
            let mut new_item: GrowthItem = old_item.into();
            let growth_plant = plants
                .iter()
                .find(|pl| pl.plant_name == new_item.plant)
                .ok_or(Error::PlantNotFound(new_item.plant.clone()))?;
            let health = growth_plant.plant_health.parse::<i32>()?;
            if health > 5 || health < 0 {
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
        if !growth_file_out.exists() {
            log::info!("Creating growth log");
            File::create(growth_file_out)?;
        }
        write_csv(new, growth_file_out)?;
        Ok(())
    }
}
