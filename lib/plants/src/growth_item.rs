use super::{errors::Error, serialize::date_serializer};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap};

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

impl TryFrom<HashMap<String, String>> for GrowthItem {
    type Error = Error;
    fn try_from(map: HashMap<String, String>) -> Result<GrowthItem, Error> {
        let date_format = map.get("date_format").ok_or(Error::KeyNotFound {
            key: "date_format".to_owned(),
            task: "GrowthItem".to_owned(),
        })?;
        let lookup_fun = |key: &str| {
            map.get(key).cloned().ok_or(Error::KeyNotFound {
                key: key.to_owned(),
                task: "GrowthItem".to_owned(),
            })
        };
        let height_cm = lookup_fun("height_cm")?.parse::<f32>()?;
        let width_cm = lookup_fun("width_cm")?.parse::<f32>()?;
        let health = lookup_fun("health")?.parse::<i32>()?;
        Ok(GrowthItem {
            plant: lookup_fun("plant")?,
            date: NaiveDate::parse_from_str(&lookup_fun("date")?, &date_format)?,
            height_cm,
            width_cm,
            health,
            note: map.get("note").cloned(),
        })
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
