use super::serialize::date_serializer;
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

#[cfg(test)]
mod graveyard_tests {
    use super::GraveyardPlant;
    use crate::test_common::{example_date1, example_date2};
    use std::cmp::Ordering;

    fn example_graveyard() -> GraveyardPlant {
        GraveyardPlant {
            name: "dead plant1".to_owned(),
            species: "species".to_owned(),
            planted: example_date1(),
            died: example_date1(),
            reason: "death".to_owned(),
        }
    }

    #[test]
    fn cmp_died_eq() {
        let gr1 = example_graveyard();
        let mut gr2 = example_graveyard();
        gr2.planted = example_date2();
        let result = gr1.cmp(&gr2);
        let expected = Ordering::Less;
        assert_eq!(result, expected)
    }

    #[test]
    fn cmp_died_neq() {
        let gr1 = example_graveyard();
        let mut gr2 = example_graveyard();
        gr2.died = example_date2();
        let result = gr1.cmp(&gr2);
        let expected = Ordering::Less;
        assert_eq!(result, expected)
    }
}
