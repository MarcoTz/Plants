use super::date::date_serializer;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogItem {
    pub activity: String,
    #[serde(with = "date_serializer")]
    pub date: NaiveDate,
    pub plant: String,
    pub note: Option<String>,
}
