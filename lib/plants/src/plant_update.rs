use super::{
    errors::Error,
    plant::PlantSpecies,
    plant::{Plant, PlantLocation},
};
use chrono::NaiveDate;
use std::{fmt, str::FromStr};

#[derive(Clone)]
pub enum UpdateValue {
    Str(String),
    Location(PlantLocation),
    Species(Box<PlantSpecies>),
    Date(NaiveDate),
    Bool(bool),
    Note(Vec<String>, bool),
}

#[derive(PartialEq, Eq, Clone)]
pub enum UpdateField {
    Name,
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
        vec![UpdateField::Name, UpdateField::Origin]
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
            UpdateField::Name => frmt.write_str("Name"),
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
            "name" => Ok(UpdateField::Name),
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
            UpdateField::Name => {
                plant.info.name = st;
                Ok(())
            }
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
                plant.info.species = *sp;
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
