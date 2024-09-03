use super::errors::Error;
use database::file_backend::{errors::Error as DBError, load_json::load_json};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct BotConfig {
    pub api_key: String,
    pub white_list: Vec<i32>,
}

pub fn load_config() -> Result<BotConfig, Error> {
    let config_path = PathBuf::from("bot_conf.json");
    let conf: BotConfig = load_json(&config_path)
        .map_err(|err| <Box<DBError> as Into<Box<dyn std::error::Error>>>::into(Box::new(err)))?;
    Ok(conf)
}
