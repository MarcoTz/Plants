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
