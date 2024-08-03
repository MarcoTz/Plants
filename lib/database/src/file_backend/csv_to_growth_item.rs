use super::errors::Error;
use chrono::NaiveDate;
use plants::date::date_serializer;
use plants::growth_item::GrowthItem;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GrowthCSV {
    #[serde(with = "date_serializer")]
    date: NaiveDate,
    plant: String,
    height: f32,
    width: f32,
    note: Option<String>,
}

impl Into<GrowthItem> for GrowthCSV {
    fn into(self) -> GrowthItem {
        GrowthItem {
            date: self.date,
            height_cm: self.height,
            width_cm: self.width,
            note: self.note,
            health: 3,
        }
    }
}
