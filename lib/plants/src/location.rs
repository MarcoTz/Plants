use super::errors::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Location {
    pub name: String,
    pub outside: bool,
}

impl TryFrom<HashMap<String, String>> for Location {
    type Error = Error;

    fn try_from(map: HashMap<String, String>) -> Result<Location, Error> {
        let lookup_fun = |key: &str| {
            map.get(key).cloned().ok_or(Error::KeyNotFound {
                key: key.to_owned(),
                task: "PlantLocation".to_owned(),
            })
        };
        Ok(Location {
            name: lookup_fun("name")?,
            outside: lookup_fun("outside")? == "1",
        })
    }
}
