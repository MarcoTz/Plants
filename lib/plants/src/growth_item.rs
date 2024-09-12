use super::serialize::date_serializer;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct GrowthItem {
    pub plant: String,
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
        Some(self.cmp(other))
    }
}

impl Ord for GrowthItem {
    fn cmp(&self, other: &GrowthItem) -> Ordering {
        self.date.cmp(&other.date)
    }
}

#[cfg(test)]
mod growth_item_test {
    use crate::test_common::{example_growth1, example_growth2};
    use std::cmp::Ordering;

    #[test]
    fn cmp_items() {
        let result = example_growth1().cmp(&example_growth2());
        let expected = Ordering::Less;
        assert_eq!(result, expected)
    }
}
