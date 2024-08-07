use super::date::date_serializer;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct GraveyardPlant {
    pub name: String,
    pub species: String,
    #[serde(with = "date_serializer")]
    pub planted: NaiveDate,
    #[serde(with = "date_serializer")]
    pub died: NaiveDate,
    pub reason: String,
}

impl PartialOrd for GraveyardPlant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GraveyardPlant {
    fn cmp(&self, other: &Self) -> Ordering {
        let died_ord = self.died.cmp(&other.died);
        if died_ord == Ordering::Equal {
            self.planted.cmp(&other.planted)
        } else {
            died_ord
        }
    }
}
