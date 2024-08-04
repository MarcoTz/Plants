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

impl Into<Vec<LogItem>> for LogCSV {
    fn into(self) -> Vec<LogItem> {
        let new_plants: Vec<String> = self
            .plants
            .split(",")
            .into_iter()
            .map(|st| st.trim().to_owned())
            .collect();
        let mut items = vec![];
        for item in new_plants.iter() {
            let new_log = LogItem {
                date: self.date,
                activity: self.activity.clone(),
                plant: item.clone(),
                note: self.note.clone(),
            };
            items.push(new_log);
        }
        items
    }
}
