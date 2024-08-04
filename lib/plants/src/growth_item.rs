use super::date::date_serializer;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct GrowthItem {
    #[serde(with = "date_serializer")]
    pub date: NaiveDate,
    pub height_cm: f32,
    pub width_cm: f32,
    pub note: Option<String>,
    pub health: i32,
}

impl Eq for GrowthItem {}

impl PartialOrd for GrowthItem {
    fn partial_cmp(&self, other: &GrowthItem) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for GrowthItem {
    fn cmp(&self, other: &GrowthItem) -> Ordering {
        self.date.cmp(&other.date)
    }
}
