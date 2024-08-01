use super::errors::DBError;
use std::fs;

const PLANTS_DIR: &str = "data/Plants";

pub fn load_json(file_name: &str) -> Result<serde_json::Value, DBError> {
    let file_contents = fs::read_to_string(file_name)?;
    let res = serde_json::from_str(&file_contents)?;
    Ok(res)
}

pub fn load_plants() -> Result<Vec<serde_json::Value>, DBError> {
    let mut plant_list = vec![];
    for dir_entry in fs::read_dir(PLANTS_DIR)? {
        let entry = dir_entry?;
        let m_path = entry.path();
        let path_str = m_path
            .to_str()
            .to_owned()
            .ok_or(DBError::PathError("no clue".to_owned()))?;
        let plant_contents = load_json(path_str)?;
        plant_list.push(plant_contents);
    }
    Ok(plant_list)
}
