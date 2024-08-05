use super::date::date_serializer;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LogItem {
    pub activity: String,
    #[serde(with = "date_serializer")]
    pub date: NaiveDate,
    pub plant: String,
    pub note: Option<String>,
}

impl PartialOrd for LogItem {
    fn partial_cmp(&self, other: &LogItem) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogItem {
    fn cmp(&self, other: &LogItem) -> Ordering {
        self.date.cmp(&other.date)
    }
}
