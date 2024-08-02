use chrono::NaiveDate;

pub struct LogItem {
    activity: String,
    date: NaiveDate,
    note: String,
}
