use super::date::date_serializer;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LogItem {
    pub activity: String,
    #[serde(with = "date_serializer")]
    pub date: NaiveDate,
    pub plants: Vec<String>,
    pub note: Option<String>,
}
