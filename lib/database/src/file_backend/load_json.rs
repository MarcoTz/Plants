use super::errors::Error;
use super::json_to_plant::PlantJSON;
use super::json_to_species::SpeciesJSON;
use plants::plant::Plant;
use plants::species::Species;
use serde::de::DeserializeOwned;
use std::fs;

fn load_json<T: DeserializeOwned>(file_name: &str) -> Result<T, Error> {
    let file_contents = fs::read_to_string(file_name)?;
    let res = serde_json::from_str(&file_contents)?;
    Ok(res)
}

pub fn load_dir<T: DeserializeOwned>(dir_path: &str) -> Result<Vec<T>, Error> {
    let mut struct_list = vec![];
    for dir_entry in fs::read_dir(dir_path)? {
        let entry = dir_entry?;
        let m_path = entry.path();
        let path_str = m_path
            .to_str()
            .to_owned()
            .ok_or(Error::PathError("no clue".to_owned()))?;
        let json_contents: T = load_json(path_str)?;
        struct_list.push(json_contents);
    }
    Ok(struct_list)
}
pub fn load_plants(plants_dir: &str) -> Result<Vec<Plant>, Error> {
    let plants_old: Vec<PlantJSON> = load_dir(plants_dir)?;
    let plants_new = plants_old
        .iter()
        .cloned()
        .map(|pl| <PlantJSON as TryInto<Plant>>::try_into(pl))
        .collect::<Result<Vec<Plant>, Error>>()?;
    Ok(plants_new)
}

pub fn load_species(species_dir: &str) -> Result<Vec<Species>, Error> {
    let species_old: Vec<SpeciesJSON> = load_dir(species_dir)?;
    let species_new = species_old
        .iter()
        .cloned()
        .map(|sp| <SpeciesJSON as TryInto<Species>>::try_into(sp))
        .collect::<Result<Vec<Species>, Error>>()?;
    Ok(species_new)
}
