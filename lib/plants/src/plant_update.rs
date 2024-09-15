use super::{
    errors::Error,
    plant::PlantSpecies,
    plant::{Plant, PlantLocation},
};
use chrono::NaiveDate;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateValue {
    Str(String),
    Location(PlantLocation),
    Species(PlantSpecies),
    Date(NaiveDate),
    Bool(bool),
    Note(Vec<String>, bool),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UpdateField {
    Species,
    Location,
    Origin,
    Obtained,
    AutoWater,
    Notes,
}

impl UpdateField {
    pub fn fields_strs() -> Vec<String> {
        let mut all_fields = UpdateField::get_str_fields();
        all_fields.extend(UpdateField::get_species_fields());
        all_fields.extend(UpdateField::get_location_fields());
        all_fields.extend(UpdateField::get_date_fields());
        all_fields.extend(UpdateField::get_note_fields());
        all_fields.extend(UpdateField::get_bool_fields());
        all_fields.iter().map(|field| field.to_string()).collect()
    }

    pub fn get_str_fields() -> Vec<UpdateField> {
        vec![UpdateField::Origin]
    }
    pub fn get_location_fields() -> Vec<UpdateField> {
        vec![UpdateField::Location]
    }
    pub fn get_species_fields() -> Vec<UpdateField> {
        vec![UpdateField::Species]
    }

    pub fn get_date_fields() -> Vec<UpdateField> {
        vec![UpdateField::Obtained]
    }

    pub fn get_bool_fields() -> Vec<UpdateField> {
        vec![UpdateField::AutoWater]
    }

    pub fn get_note_fields() -> Vec<UpdateField> {
        vec![UpdateField::Notes]
    }
}

impl fmt::Display for UpdateField {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpdateField::Species => frmt.write_str("Species"),
            UpdateField::Location => frmt.write_str("Location"),
            UpdateField::Origin => frmt.write_str("Origin"),
            UpdateField::Obtained => frmt.write_str("Obtained"),
            UpdateField::AutoWater => frmt.write_str("Auto Watered"),
            UpdateField::Notes => frmt.write_str("Notes"),
        }
    }
}

impl FromStr for UpdateField {
    type Err = Error;
    fn from_str(s: &str) -> Result<UpdateField, Self::Err> {
        let s_clean = s.trim().to_lowercase().replace(' ', "");
        match s_clean.as_str() {
            "species" => Ok(UpdateField::Species),
            "location" => Ok(UpdateField::Location),
            "origin" => Ok(UpdateField::Origin),
            "obtained" => Ok(UpdateField::Obtained),
            "autowatered" => Ok(UpdateField::AutoWater),
            "notes" => Ok(UpdateField::Notes),
            _ => Err(Error::FieldError(s.to_owned())),
        }
    }
}

pub fn update_plant(
    plant: &mut Plant,
    field: UpdateField,
    value: UpdateValue,
) -> Result<(), Error> {
    let field_err = Error::WrongType(field.to_string());
    match value {
        UpdateValue::Str(st) => match field {
            UpdateField::Origin => {
                plant.info.origin = st;
                Ok(())
            }
            _ => Err(field_err),
        },
        UpdateValue::Location(loc) => {
            if let UpdateField::Location = field {
                plant.info.location = loc;
                Ok(())
            } else {
                Err(Error::FieldError(field.to_string()))
            }
        }
        UpdateValue::Species(sp) => {
            if let UpdateField::Species = field {
                plant.info.species = sp;
                Ok(())
            } else {
                Err(Error::FieldError(field.to_string()))
            }
        }
        UpdateValue::Date(dt) => {
            if let UpdateField::Obtained = field {
                plant.info.obtained = dt;
                Ok(())
            } else {
                Err(field_err)
            }
        }
        UpdateValue::Bool(b) => {
            if let UpdateField::AutoWater = field {
                plant.info.auto_water = b;
                Ok(())
            } else {
                Err(field_err)
            }
        }
        UpdateValue::Note(notes, append) => {
            if let UpdateField::Notes = field {
                if append {
                    plant.info.notes.extend(notes);
                } else {
                    plant.info.notes = notes;
                }
                Ok(())
            } else {
                Err(field_err)
            }
        }
    }
}

#[cfg(test)]
mod plant_udpate_tests {
    use super::{update_plant, UpdateField, UpdateValue};
    use crate::{
        plant::{PlantLocation, PlantSpecies},
        test_common::{example_date2, example_plant},
    };
    use std::str::FromStr;

    #[test]
    fn field_strs() {
        let result = UpdateField::fields_strs();
        let expected = vec![
            "Name".to_owned(),
            "Origin".to_owned(),
            "Species".to_owned(),
            "Location".to_owned(),
            "Obtained".to_owned(),
            "Notes".to_owned(),
            "Auto Watered".to_owned(),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn str_fields() {
        let result = UpdateField::get_str_fields();
        let expected = vec![UpdateField::Origin];
        assert_eq!(result, expected)
    }

    #[test]
    fn location_fields() {
        let result = UpdateField::get_location_fields();
        let expected = vec![UpdateField::Location];
        assert_eq!(result, expected)
    }

    #[test]
    fn species_fields() {
        let result = UpdateField::get_species_fields();
        let expected = vec![UpdateField::Species];
        assert_eq!(result, expected)
    }

    #[test]
    fn date_fields() {
        let result = UpdateField::get_date_fields();
        let expected = vec![UpdateField::Obtained];
        assert_eq!(result, expected)
    }

    #[test]
    fn bool_fields() {
        let result = UpdateField::get_bool_fields();
        let expected = vec![UpdateField::AutoWater];
        assert_eq!(result, expected)
    }

    #[test]
    fn note_fields() {
        let result = UpdateField::get_note_fields();
        let expected = vec![UpdateField::Notes];
        assert_eq!(result, expected)
    }

    #[test]
    fn species_field() {
        let result = UpdateField::from_str("species").unwrap();
        let expected = UpdateField::Species;
        assert_eq!(result, expected)
    }

    #[test]
    fn location_field() {
        let result = UpdateField::from_str("location").unwrap();
        let expected = UpdateField::Location;
        assert_eq!(result, expected)
    }

    #[test]
    fn origin_field() {
        let result = UpdateField::from_str("origin").unwrap();
        let expected = UpdateField::Origin;
        assert_eq!(result, expected)
    }

    #[test]
    fn obtained_field() {
        let result = UpdateField::from_str("obtained").unwrap();
        let expected = UpdateField::Obtained;
        assert_eq!(result, expected)
    }

    #[test]
    fn autowater_field() {
        let result = UpdateField::from_str("auto watered").unwrap();
        let expected = UpdateField::AutoWater;
        assert_eq!(result, expected)
    }

    #[test]
    fn notes_field() {
        let result = UpdateField::from_str("notes").unwrap();
        let expected = UpdateField::Notes;
        assert_eq!(result, expected)
    }

    #[test]
    fn str_fail() {
        let result = UpdateField::from_str("other");
        assert!(result.is_err())
    }

    #[test]
    fn update_origin() {
        let mut result = example_plant();
        let mut expected = example_plant();
        update_plant(
            &mut result,
            UpdateField::Origin,
            UpdateValue::Str("new origin".to_owned()),
        )
        .unwrap();
        expected.info.origin = "new origin".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn update_str_err() {
        let result = update_plant(
            &mut example_plant(),
            UpdateField::Obtained,
            UpdateValue::Bool(false),
        );
        assert!(result.is_err())
    }

    #[test]
    fn update_loc() {
        let mut result = example_plant();
        let mut expected = example_plant();
        update_plant(
            &mut result,
            UpdateField::Location,
            UpdateValue::Location(PlantLocation::Other("new location".to_owned())),
        )
        .unwrap();
        expected.info.location = PlantLocation::Other("new location".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn update_loc_err() {
        let result = update_plant(
            &mut example_plant(),
            UpdateField::Location,
            UpdateValue::Bool(false),
        );
        assert!(result.is_err())
    }

    #[test]
    fn update_species() {
        let mut result = example_plant();
        let mut expected = example_plant();
        update_plant(
            &mut result,
            UpdateField::Species,
            UpdateValue::Species(PlantSpecies::Other("new species".to_owned())),
        )
        .unwrap();
        expected.info.species = PlantSpecies::Other("new species".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn update_species_err() {
        let result = update_plant(
            &mut example_plant(),
            UpdateField::Species,
            UpdateValue::Bool(false),
        );
        assert!(result.is_err())
    }

    #[test]
    fn update_obtained() {
        let mut result = example_plant();
        let mut expected = example_plant();
        update_plant(
            &mut result,
            UpdateField::Obtained,
            UpdateValue::Date(example_date2()),
        )
        .unwrap();
        expected.info.obtained = example_date2();
        assert_eq!(result, expected)
    }

    #[test]
    fn update_obtained_err() {
        let result = update_plant(
            &mut example_plant(),
            UpdateField::Obtained,
            UpdateValue::Bool(false),
        );
        assert!(result.is_err())
    }

    #[test]
    fn update_auto_water() {
        let mut result = example_plant();
        let mut expected = example_plant();
        update_plant(&mut result, UpdateField::AutoWater, UpdateValue::Bool(true)).unwrap();
        expected.info.auto_water = true;
        assert_eq!(result, expected)
    }

    #[test]
    fn update_auto_water_err() {
        let result = update_plant(
            &mut example_plant(),
            UpdateField::AutoWater,
            UpdateValue::Str("".to_owned()),
        );
        assert!(result.is_err())
    }

    #[test]
    fn update_notes_append() {
        let mut result = example_plant();
        let mut expected = example_plant();
        update_plant(
            &mut result,
            UpdateField::Notes,
            UpdateValue::Note(vec!["new note".to_owned()], true),
        )
        .unwrap();
        expected.info.notes.push("new note".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn update_notes_replace() {
        let mut result = example_plant();
        let mut expected = example_plant();
        update_plant(
            &mut result,
            UpdateField::Notes,
            UpdateValue::Note(vec!["new note".to_owned()], true),
        )
        .unwrap();
        expected.info.notes = vec!["new note".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn update_notes_err() {
        let result = update_plant(
            &mut example_plant(),
            UpdateField::Notes,
            UpdateValue::Bool(false),
        );
        assert!(result.is_err())
    }
}
