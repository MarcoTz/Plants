use chrono::NaiveDate;

pub struct GraveyardPlant {
    name: String,
    species: String,
    planted: NaiveDate,
    died: NaiveDate,
    reason: String,
}
