use super::{
    errors::Error,
    species::{Species, SunlightRequirement},
};
use std::{fmt, str::FromStr};

#[derive(Clone)]
pub enum UpdateValue {
    Str(String),
    Sun(SunlightRequirement),
    Fl(f32),
    MFl(Option<f32>),
    Note(Vec<String>, bool),
    MInt(Option<i32>),
}

#[derive(PartialEq, Eq, Clone)]
pub enum UpdateField {
    Name,
    ScientificName,
    Genus,
    Family,
    Sunlight,
    TempMin,
    TempMax,
    TempMinOpt,
    TempMaxOpt,
    PlantingDistance,
    PhMin,
    PhMax,
    WateringNotes,
    AvgWateringDays,
    FertilizingNotes,
    AvgFertilizingDays,
    PruningNotes,
    Companions,
    AdditionalNotes,
}

impl UpdateField {
    pub fn fields_strs() -> Vec<String> {
        let mut all_fields = UpdateField::get_str_fields();
        all_fields.extend(UpdateField::get_sun_fields());
        all_fields.extend(UpdateField::get_fl_fields());
        all_fields.extend(UpdateField::get_note_fields());
        all_fields.extend(UpdateField::get_mint_fields());
        all_fields.iter().map(|field| field.to_string()).collect()
    }
    pub fn get_str_fields() -> Vec<UpdateField> {
        vec![
            UpdateField::Name,
            UpdateField::ScientificName,
            UpdateField::Genus,
            UpdateField::Family,
        ]
    }
    fn get_sun_fields() -> Vec<UpdateField> {
        vec![UpdateField::Sunlight]
    }
    fn get_fl_fields() -> Vec<UpdateField> {
        vec![
            UpdateField::TempMin,
            UpdateField::TempMax,
            UpdateField::TempMinOpt,
            UpdateField::TempMaxOpt,
            UpdateField::PhMin,
            UpdateField::PhMax,
        ]
    }
    fn get_mfl_fields() -> Vec<UpdateField> {
        vec![UpdateField::PlantingDistance]
    }
    fn get_note_fields() -> Vec<UpdateField> {
        vec![
            UpdateField::WateringNotes,
            UpdateField::FertilizingNotes,
            UpdateField::PruningNotes,
            UpdateField::Companions,
            UpdateField::AdditionalNotes,
        ]
    }
    fn get_mint_fields() -> Vec<UpdateField> {
        vec![
            UpdateField::AvgWateringDays,
            UpdateField::AvgFertilizingDays,
        ]
    }
}

impl TryFrom<(String, &UpdateField)> for UpdateValue {
    type Error = Error;
    fn try_from((s, field): (String, &UpdateField)) -> Result<UpdateValue, Self::Error> {
        let ty_err = Error::WrongType(field.to_string());
        if UpdateField::get_str_fields().contains(field) {
            Ok(UpdateValue::Str(s.trim().to_owned()))
        } else if UpdateField::get_fl_fields().contains(field) {
            let fl = s.parse::<f32>().map_err(|_| ty_err)?;
            Ok(UpdateValue::Fl(fl))
        } else if UpdateField::get_sun_fields().contains(field) {
            let sun = s.parse::<SunlightRequirement>().map_err(|_| ty_err)?;
            Ok(UpdateValue::Sun(sun))
        } else if UpdateField::get_mfl_fields().contains(field) {
            let fl = s.parse::<f32>().map_err(|_| ty_err)?;
            let value = if fl < 0.0 {
                UpdateValue::MFl(None)
            } else {
                UpdateValue::MFl(Some(fl))
            };
            Ok(value)
        } else if UpdateField::get_note_fields().contains(field) {
            let notes = s.split(',').map(|st| st.trim().to_owned()).collect();
            Ok(UpdateValue::Note(notes, true))
        } else if UpdateField::get_mint_fields().contains(field) {
            let i = s.parse::<i32>().map_err(|_| ty_err)?;
            let value = if i < 0 {
                UpdateValue::MInt(None)
            } else {
                UpdateValue::MInt(Some(i))
            };
            Ok(value)
        } else {
            Err(ty_err)
        }
    }
}

impl fmt::Display for UpdateField {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpdateField::Name => frmt.write_str("Name"),
            UpdateField::ScientificName => frmt.write_str("Scientific Name"),
            UpdateField::Genus => frmt.write_str("Genus"),
            UpdateField::Family => frmt.write_str("Family"),
            UpdateField::Sunlight => frmt.write_str("Sunlight"),
            UpdateField::TempMin => frmt.write_str("Min Temp"),
            UpdateField::TempMax => frmt.write_str("Max Temp"),
            UpdateField::TempMinOpt => frmt.write_str("Min Temp Opt"),
            UpdateField::TempMaxOpt => frmt.write_str("Max Temp Opt"),
            UpdateField::PlantingDistance => frmt.write_str("Planting Distance"),
            UpdateField::PhMin => frmt.write_str("pH Min"),
            UpdateField::PhMax => frmt.write_str("pH Max"),
            UpdateField::WateringNotes => frmt.write_str("Watering Notes"),
            UpdateField::AvgWateringDays => frmt.write_str("Average Watering Days"),
            UpdateField::FertilizingNotes => frmt.write_str("Ferilizing Notes"),
            UpdateField::AvgFertilizingDays => frmt.write_str("Average Fertilizing Days"),
            UpdateField::PruningNotes => frmt.write_str("Pruning Notes"),
            UpdateField::Companions => frmt.write_str("Companions"),
            UpdateField::AdditionalNotes => frmt.write_str("Additinal Notes"),
        }
    }
}

impl FromStr for UpdateField {
    type Err = Error;
    fn from_str(s: &str) -> Result<UpdateField, Self::Err> {
        let s_clean = s.trim().to_lowercase().replace(' ', "");
        match s_clean.as_str() {
            "name" => Ok(UpdateField::Name),
            "scientificname" => Ok(UpdateField::ScientificName),
            "genus" => Ok(UpdateField::Genus),
            "family" => Ok(UpdateField::Family),
            "sunlight" => Ok(UpdateField::Sunlight),
            "mintemp" => Ok(UpdateField::TempMin),
            "maxtemp" => Ok(UpdateField::TempMax),
            "mintempopt" => Ok(UpdateField::TempMinOpt),
            "maxtempopt" => Ok(UpdateField::TempMaxOpt),
            "plantingdistance" => Ok(UpdateField::PlantingDistance),
            "phmin" => Ok(UpdateField::PhMin),
            "phmax" => Ok(UpdateField::PhMax),
            "wateringnotes" => Ok(UpdateField::WateringNotes),
            "averagewateringdays" => Ok(UpdateField::AvgWateringDays),
            "ferilizingnotes" => Ok(UpdateField::FertilizingNotes),
            "averagefertilizing days" => Ok(UpdateField::AvgFertilizingDays),
            "pruningnotes" => Ok(UpdateField::PruningNotes),
            "companions" => Ok(UpdateField::Companions),
            "additinalnotes" => Ok(UpdateField::AdditionalNotes),
            _ => Err(Error::FieldError(s.to_owned())),
        }
    }
}

pub fn update_species(
    species: &mut Species,
    field: UpdateField,
    value: UpdateValue,
) -> Result<(), Error> {
    let field_err = Error::WrongType(field.to_string());
    match value {
        UpdateValue::Str(st) => match field {
            UpdateField::Name => {
                species.name = st;
                Ok(())
            }
            UpdateField::ScientificName => {
                species.scientific_name = st;
                Ok(())
            }
            UpdateField::Genus => {
                species.genus = st;
                Ok(())
            }
            UpdateField::Family => {
                species.family = st;
                Ok(())
            }
            _ => Err(field_err),
        },
        UpdateValue::Sun(sun) => {
            if let UpdateField::Sunlight = field {
                species.sunlight = sun;
                Ok(())
            } else {
                Err(Error::FieldError(field.to_string()))
            }
        }
        UpdateValue::Fl(fl) => match field {
            UpdateField::TempMin => {
                species.temp_min = fl;
                Ok(())
            }
            UpdateField::TempMax => {
                species.temp_max = fl;
                Ok(())
            }
            UpdateField::TempMinOpt => {
                species.opt_temp_min = fl;
                Ok(())
            }
            UpdateField::TempMaxOpt => {
                species.opt_temp_max = fl;
                Ok(())
            }
            UpdateField::PhMin => {
                species.ph_min = fl;
                Ok(())
            }
            UpdateField::PhMax => {
                species.ph_max = fl;
                Ok(())
            }
            _ => Err(field_err),
        },
        UpdateValue::MFl(mfl) => {
            if let UpdateField::PlantingDistance = field {
                species.planting_distance = mfl;
                Ok(())
            } else {
                Err(field_err)
            }
        }
        UpdateValue::Note(notes, append) => match field {
            UpdateField::WateringNotes => {
                if append {
                    species.watering_notes.extend(notes);
                    Ok(())
                } else {
                    species.watering_notes = notes;
                    Ok(())
                }
            }
            UpdateField::FertilizingNotes => {
                if append {
                    species.fertilizing_notes.extend(notes);
                    Ok(())
                } else {
                    species.fertilizing_notes = notes;
                    Ok(())
                }
            }
            UpdateField::PruningNotes => {
                if append {
                    species.pruning_notes.extend(notes);
                    Ok(())
                } else {
                    species.pruning_notes = notes;
                    Ok(())
                }
            }
            UpdateField::Companions => {
                if append {
                    species.companions.extend(notes);
                    Ok(())
                } else {
                    species.companions = notes;
                    Ok(())
                }
            }
            UpdateField::AdditionalNotes => {
                if append {
                    species.additional_notes.extend(notes);
                    Ok(())
                } else {
                    species.additional_notes = notes;
                    Ok(())
                }
            }
            _ => Err(field_err),
        },
        UpdateValue::MInt(mi) => match field {
            UpdateField::AvgWateringDays => {
                species.avg_watering_days = mi;
                Ok(())
            }
            UpdateField::AvgFertilizingDays => {
                species.avg_fertilizing_days = mi;
                Ok(())
            }
            _ => Err(field_err),
        },
    }
}
