use crate::errors::Error;
use chrono::NaiveDate;
use database::database_manager::DatabaseManager;
use plants::{
    plant::PlantSpecies,
    plant_update::{UpdateField, UpdateValue},
};

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

pub fn str_to_value<T: DatabaseManager>(
    input: String,
    field: &UpdateField,
    db_man: &mut T,
    date_format: &str,
) -> Result<UpdateValue, Error> {
    let ty_err = Error::ParseError(format!("Plant Update {}, with value {}", field, input));
    if UpdateField::get_str_fields().contains(field) {
        Ok(UpdateValue::Str(input.trim().to_owned()))
    } else if UpdateField::get_species_fields().contains(field) {
        let species = db_man.get_species(input.trim());
        match species {
            Ok(sp) => Ok(UpdateValue::Species(Box::new(PlantSpecies::Species(
                Box::new(sp),
            )))),
            Err(_) => Ok(UpdateValue::Species(Box::new(PlantSpecies::Other(input)))),
        }
    } else if UpdateField::get_date_fields().contains(field) {
        let date = NaiveDate::parse_from_str(&input, date_format).map_err(|_| ty_err)?;
        Ok(UpdateValue::Date(date))
    } else if UpdateField::get_bool_fields().contains(field) {
        let b = input.trim().parse::<bool>().map_err(|_| ty_err)?;
        Ok(UpdateValue::Bool(b))
    } else if UpdateField::get_note_fields().contains(field) {
        let notes = input.split(',').map(|nt| nt.trim().to_owned()).collect();
        Ok(UpdateValue::Note(notes, true))
    } else {
        Err(ty_err)
    }
}
