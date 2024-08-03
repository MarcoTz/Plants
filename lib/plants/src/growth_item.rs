use super::date::date_serializer;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GrowthItem {
    #[serde(with = "date_serializer")]
    pub date: NaiveDate,
    pub height_cm: f32,
    pub width_cm: f32,
    pub note: Option<String>,
    pub health: i32,
}
