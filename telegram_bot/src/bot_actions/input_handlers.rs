use crate::errors::Error;
use database::database_manager::DatabaseManager;

pub fn input_plant_name<T: DatabaseManager>(
    input: String,
    db_man: &mut T,
) -> Result<String, Error> {
    let name = input.trim().to_owned();
    let exists = db_man.plant_exists(&name)?;
    if exists {
        Ok(name)
    } else {
        Err(Error::PlantDoesNotExist(name))
    }
}

pub fn input_plant_names<T: DatabaseManager>(
    input: String,
    db_man: &mut T,
) -> Result<Vec<String>, Error> {
    let mut plants = vec![];
    for plant_name in input.split(',').collect::<Vec<&str>>().into_iter() {
        let name = input_plant_name(plant_name.to_owned(), db_man)?;
        plants.push(name);
    }
    Ok(plants)
}

pub fn input_health(input: String) -> Result<i32, Error> {
    let health = input
        .trim()
        .parse::<i32>()
        .map_err(|_| Error::ParseError("Health".to_owned()))?;
    if !(0..=5).contains(&health) {
        Err(Error::BadHealth(health))
    } else {
        Ok(health)
    }
}

pub fn input_species<T: DatabaseManager>(input: String, db_man: &mut T) -> Result<String, Error> {
    let name = input.trim().to_owned();
    let exists = db_man.species_exists(&name)?;
    if exists {
        Ok(name)
    } else {
        Err(Error::SpeciesDoesNotExist(name))
    }
}

pub fn input_notes(input: String) -> Vec<String> {
    if input.trim().to_lowercase() == "done" {
        vec![]
    } else {
        input.split(',').map(|st| st.trim().to_owned()).collect()
    }
}
