use super::errors::Error;
use chrono::NaiveDate;
use plants::date::date_serializer;
use plants::log_item::LogItem;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct LogCSV {
    #[serde(with = "date_serializer")]
    date: NaiveDate,
    activity: String,
    plants: String,
    note: Option<String>,
}

impl Into<LogItem> for LogCSV {
    fn into(self) -> LogItem {
        let new_plants = self
            .plants
            .split(",")
            .into_iter()
            .map(|st| st.trim().to_owned())
            .collect();
        LogItem {
            date: self.date,
            activity: self.activity,
            plants: new_plants,
            note: self.note,
        }
    }
}
