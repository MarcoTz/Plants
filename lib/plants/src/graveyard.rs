use super::date::date_serializer;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GraveyardPlant {
    name: String,
    species: String,
    #[serde(with = "date_serializer")]
    planted: NaiveDate,
    #[serde(with = "date_serializer")]
    died: NaiveDate,
    reason: String,
}
