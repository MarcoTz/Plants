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

impl From<LogCSV> for Vec<LogItem> {
    fn from(log_csv: LogCSV) -> Vec<LogItem> {
        let new_plants: Vec<String> = log_csv
            .plants
            .split(',')
            .map(|st| st.trim().to_owned())
            .collect();
        let mut items = vec![];
        for item in new_plants.iter() {
            let new_log = LogItem {
                date: log_csv.date,
                activity: log_csv.activity.clone(),
                plant: item.clone(),
                note: log_csv.note.clone(),
            };
            items.push(new_log);
        }
        items
    }
}
