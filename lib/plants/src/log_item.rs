use super::serialize::date_serializer;
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
