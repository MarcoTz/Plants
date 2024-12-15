use chrono::NaiveDate;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlantImage {
    pub created: NaiveDate,
    pub file_name: String,
    pub file_path: PathBuf,
}
