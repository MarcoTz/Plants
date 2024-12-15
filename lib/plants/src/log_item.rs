use super::{errors::Error, serialize::date_serializer};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap};

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

impl TryFrom<HashMap<String, String>> for LogItem {
    type Error = Error;
    fn try_from(map: HashMap<String, String>) -> Result<LogItem, Error> {
        let date_format = map.get("date_format").ok_or(Error::KeyNotFound {
            key: "date_format".to_owned(),
            task: "LogItem".to_owned(),
        })?;
        let lookup_fun = |key: &str| {
            map.get(key).cloned().ok_or(Error::KeyNotFound {
                key: key.to_owned(),
                task: "LogItem".to_owned(),
            })
        };
        Ok(LogItem {
            activity: lookup_fun("name")?,
            date: NaiveDate::parse_from_str(&lookup_fun("date")?, &date_format)?,
            plant: lookup_fun("plant")?,
            note: map.get("note").cloned(),
        })
    }
}
#[cfg(test)]
mod location_tests {
    use crate::test_common::{example_activity1, example_activity2};
    use std::cmp::Ordering;

    #[test]
    fn compare_items() {
        let result = example_activity1().cmp(&example_activity2());
        let expected = Ordering::Less;
        assert_eq!(result, expected)
    }
}
