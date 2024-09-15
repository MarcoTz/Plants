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
            Ok(sp) => Ok(UpdateValue::Species(PlantSpecies::Species(Box::new(sp)))),
            Err(_) => Ok(UpdateValue::Species(PlantSpecies::Other(input))),
        }
    } else if UpdateField::get_date_fields().contains(field) {
        let date = NaiveDate::parse_from_str(&input, date_format).map_err(|_| ty_err)?;
        Ok(UpdateValue::Date(date))
    } else if UpdateField::get_bool_fields().contains(field) {
        let inp = input.trim().to_lowercase();
        let b = if inp == "y" || inp == "true" {
            Ok(true)
        } else if inp == "n" || inp == "false" {
            Ok(false)
        } else {
            Err(ty_err)
        }?;
        Ok(UpdateValue::Bool(b))
    } else if UpdateField::get_note_fields().contains(field) {
        let notes = input.split(',').map(|nt| nt.trim().to_owned()).collect();
        Ok(UpdateValue::Note(notes, true))
    } else {
        Err(ty_err)
    }
}

#[cfg(test)]
mod input_handlers_tests {

    use super::{
        input_health, input_notes, input_plant_name, input_plant_names, input_species,
        str_to_value, PlantSpecies, UpdateField, UpdateValue,
    };
    use crate::test_common::{example_species, DummyManager};
    use chrono::NaiveDate;

    #[test]
    fn input_plant() {
        let result = input_plant_name("Plant1".to_owned(), &mut DummyManager {}).unwrap();
        let expected = "Plant1";
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plant_err() {
        let result = input_plant_name("not a plant".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_names() {
        let result = input_plant_names("Plant1,Plant2".to_owned(), &mut DummyManager {}).unwrap();
        let expected = vec!["Plant1".to_owned(), "Plant2".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn input_names_fail() {
        let result = input_plant_names("Plant1, not a plant".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_hlth() {
        let result = input_health("3".to_owned()).unwrap();
        let expected = 3;
        assert_eq!(result, expected)
    }

    #[test]
    fn input_health_err() {
        let result = input_health("6".to_owned());
        assert!(result.is_err())
    }

    #[test]
    fn input_sp() {
        let result = input_species("Species1".to_owned(), &mut DummyManager {}).unwrap();
        let expected = "Species1".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn input_species_fail() {
        let result = input_species("not a species".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_note() {
        let result = input_notes("note1,note2".to_owned());
        let expected = vec!["note1".to_owned(), "note2".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn input_value_str() {
        let result = str_to_value(
            "string".to_owned(),
            &UpdateField::Name,
            &mut DummyManager {},
            "%d.%m.%Y",
        )
        .unwrap();
        let expected = UpdateValue::Str("string".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_value_species() {
        let result = str_to_value(
            "Species1".to_owned(),
            &UpdateField::Species,
            &mut DummyManager {},
            "%d.%m.%Y",
        )
        .unwrap();
        let expected = UpdateValue::Species(PlantSpecies::Species(Box::new(example_species())));
        assert_eq!(result, expected)
    }

    #[test]
    fn input_value_species_other() {
        let result = str_to_value(
            "not a species".to_owned(),
            &UpdateField::Species,
            &mut DummyManager {},
            "%d.%m.%Y",
        )
        .unwrap();
        let expected = UpdateValue::Species(PlantSpecies::Other("not a species".to_owned()));
        assert_eq!(result, expected)
    }

    #[test]
    fn input_value_date() {
        let result = str_to_value(
            "01.01.1970".to_owned(),
            &UpdateField::Obtained,
            &mut DummyManager {},
            "%d.%m.%Y",
        )
        .unwrap();
        let expected =
            UpdateValue::Date(NaiveDate::parse_from_str("01.01.1970", "%d.%m.%Y").unwrap());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_value_date_err() {
        let result = str_to_value(
            "not a date".to_owned(),
            &UpdateField::Obtained,
            &mut DummyManager {},
            "%d.%m.%Y",
        );
        assert!(result.is_err())
    }

    #[test]
    fn input_value_bool() {
        let result = str_to_value(
            "true".to_owned(),
            &UpdateField::AutoWater,
            &mut DummyManager {},
            "%d.%m.%Y",
        )
        .unwrap();
        let expected = UpdateValue::Bool(true);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_value_bool_err() {
        let result = str_to_value(
            "not a bool".to_owned(),
            &UpdateField::AutoWater,
            &mut DummyManager {},
            "%d.%m.%Y",
        );
        assert!(result.is_err())
    }

    #[test]
    fn input_value_notes() {
        let result = str_to_value(
            "note1,note2".to_owned(),
            &UpdateField::Notes,
            &mut DummyManager {},
            "%d.%m.%Y",
        )
        .unwrap();
        let expected = UpdateValue::Note(vec!["note1".to_owned(), "note2".to_owned()], true);
        assert_eq!(result, expected)
    }
}
