use super::{
    errors::Error,
    species::{Species, SunlightRequirement},
};
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum UpdateValue {
    Str(String),
    Sun(SunlightRequirement),
    Fl(f32),
    MFl(Option<f32>),
    Note(Vec<String>, bool),
    MInt(Option<i32>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UpdateField {
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
        all_fields.extend(UpdateField::get_mfl_fields());
        all_fields.extend(UpdateField::get_note_fields());
        all_fields.extend(UpdateField::get_mint_fields());
        all_fields.iter().map(|field| field.to_string()).collect()
    }

    pub fn get_str_fields() -> Vec<UpdateField> {
        vec![
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
            UpdateField::FertilizingNotes => frmt.write_str("Fertilizing Notes"),
            UpdateField::AvgFertilizingDays => frmt.write_str("Average Fertilizing Days"),
            UpdateField::PruningNotes => frmt.write_str("Pruning Notes"),
            UpdateField::Companions => frmt.write_str("Companions"),
            UpdateField::AdditionalNotes => frmt.write_str("Additional Notes"),
        }
    }
}

impl FromStr for UpdateField {
    type Err = Error;
    fn from_str(s: &str) -> Result<UpdateField, Self::Err> {
        let s_clean = s.trim().to_lowercase().replace(' ', "");
        match s_clean.as_str() {
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
            "fertilizingnotes" => Ok(UpdateField::FertilizingNotes),
            "averagefertilizingdays" => Ok(UpdateField::AvgFertilizingDays),
            "pruningnotes" => Ok(UpdateField::PruningNotes),
            "companions" => Ok(UpdateField::Companions),
            "additionalnotes" => Ok(UpdateField::AdditionalNotes),
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

#[cfg(test)]
mod update_species_tests {
    use super::{update_species, UpdateField, UpdateValue};
    use crate::{species::SunlightRequirement, test_common::example_species};
    use std::str::FromStr;

    #[test]
    fn field_strs() {
        let result = UpdateField::fields_strs();
        let expected = vec![
            "Name".to_owned(),
            "Scientific Name".to_owned(),
            "Genus".to_owned(),
            "Family".to_owned(),
            "Sunlight".to_owned(),
            "Min Temp".to_owned(),
            "Max Temp".to_owned(),
            "Min Temp Opt".to_owned(),
            "Max Temp Opt".to_owned(),
            "pH Min".to_owned(),
            "pH Max".to_owned(),
            "Planting Distance".to_owned(),
            "Watering Notes".to_owned(),
            "Fertilizing Notes".to_owned(),
            "Pruning Notes".to_owned(),
            "Companions".to_owned(),
            "Additional Notes".to_owned(),
            "Average Watering Days".to_owned(),
            "Average Fertilizing Days".to_owned(),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn str_fields() {
        let result = UpdateField::get_str_fields();
        let expected = vec![
            UpdateField::ScientificName,
            UpdateField::Genus,
            UpdateField::Family,
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn sun_fields() {
        let result = UpdateField::get_sun_fields();
        let expected = vec![UpdateField::Sunlight];
        assert_eq!(result, expected)
    }

    #[test]
    fn fl_fields() {
        let result = UpdateField::get_fl_fields();
        let expected = vec![
            UpdateField::TempMin,
            UpdateField::TempMax,
            UpdateField::TempMinOpt,
            UpdateField::TempMaxOpt,
            UpdateField::PhMin,
            UpdateField::PhMax,
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn mfl_fields() {
        let result = UpdateField::get_mfl_fields();
        let expected = vec![UpdateField::PlantingDistance];
        assert_eq!(result, expected)
    }

    #[test]
    fn note_fields() {
        let result = UpdateField::get_note_fields();
        let expected = vec![
            UpdateField::WateringNotes,
            UpdateField::FertilizingNotes,
            UpdateField::PruningNotes,
            UpdateField::Companions,
            UpdateField::AdditionalNotes,
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn mint_fields() {
        let result = UpdateField::get_mint_fields();
        let expected = vec![
            UpdateField::AvgWateringDays,
            UpdateField::AvgFertilizingDays,
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn into_fl_float_err() {
        let result = UpdateValue::try_from(("not a float".to_owned(), &UpdateField::TempMax));
        assert!(result.is_err())
    }

    #[test]
    fn into_fl() {
        let result = UpdateValue::try_from(("1.0".to_owned(), &UpdateField::TempMin)).unwrap();
        let expected = UpdateValue::Fl(1.0);
        assert_eq!(result, expected)
    }

    #[test]
    fn into_sun_err() {
        let result = UpdateValue::try_from(("not sun".to_owned(), &UpdateField::Sunlight));
        assert!(result.is_err())
    }

    #[test]
    fn into_sun() {
        let result = UpdateValue::try_from(("Direct".to_owned(), &UpdateField::Sunlight)).unwrap();
        let expected = UpdateValue::Sun(SunlightRequirement::Direct);
        assert_eq!(result, expected)
    }

    #[test]
    fn into_mfl_err() {
        let result =
            UpdateValue::try_from(("not a float".to_owned(), &UpdateField::PlantingDistance));
        assert!(result.is_err())
    }

    #[test]
    fn into_mfl_none() {
        let result =
            UpdateValue::try_from(("-1.0".to_owned(), &UpdateField::PlantingDistance)).unwrap();
        let expected = UpdateValue::MFl(None);
        assert_eq!(result, expected)
    }

    #[test]
    fn into_mfl_some() {
        let result =
            UpdateValue::try_from(("1.0".to_owned(), &UpdateField::PlantingDistance)).unwrap();
        let expected = UpdateValue::MFl(Some(1.0));
        assert_eq!(result, expected)
    }

    #[test]
    fn into_notes() {
        let result =
            UpdateValue::try_from(("note".to_owned(), &UpdateField::WateringNotes)).unwrap();
        let expected = UpdateValue::Note(vec!["note".to_owned()], true);
        assert_eq!(result, expected)
    }

    #[test]
    fn into_mint_err() {
        let result =
            UpdateValue::try_from(("not an int".to_owned(), &UpdateField::AvgWateringDays));
        assert!(result.is_err())
    }

    #[test]
    fn into_mint_some() {
        let result =
            UpdateValue::try_from(("1".to_owned(), &UpdateField::AvgFertilizingDays)).unwrap();
        let expected = UpdateValue::MInt(Some(1));
        assert_eq!(result, expected)
    }

    #[test]
    fn into_mint_none() {
        let result =
            UpdateValue::try_from(("-1".to_owned(), &UpdateField::AvgFertilizingDays)).unwrap();
        let expected = UpdateValue::MInt(None);
        assert_eq!(result, expected)
    }

    #[test]
    fn display_scientific_name() {
        let result = format!("{}", UpdateField::ScientificName);
        let expected = "Scientific Name";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_genus() {
        let result = format!("{}", UpdateField::Genus);
        let expected = "Genus";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_family() {
        let result = format!("{}", UpdateField::Family);
        let expected = "Family";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_sunlight() {
        let result = format!("{}", UpdateField::Sunlight);
        let expected = "Sunlight";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_tempmin() {
        let result = format!("{}", UpdateField::TempMin);
        let expected = "Min Temp";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_tempmax() {
        let result = format!("{}", UpdateField::TempMax);
        let expected = "Max Temp";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_opttempmin() {
        let result = format!("{}", UpdateField::TempMinOpt);
        let expected = "Min Temp Opt";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_opttempmax() {
        let result = format!("{}", UpdateField::TempMaxOpt);
        let expected = "Max Temp Opt";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_plantdist() {
        let result = format!("{}", UpdateField::PlantingDistance);
        let expected = "Planting Distance";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_phmin() {
        let result = format!("{}", UpdateField::PhMin);
        let expected = "pH Min";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_phmax() {
        let result = format!("{}", UpdateField::PhMax);
        let expected = "pH Max";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_waternotes() {
        let result = format!("{}", UpdateField::WateringNotes);
        let expected = "Watering Notes";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_waterdays() {
        let result = format!("{}", UpdateField::AvgWateringDays);
        let expected = "Average Watering Days";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_fertnotes() {
        let result = format!("{}", UpdateField::FertilizingNotes);
        let expected = "Fertilizing Notes";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_fertdays() {
        let result = format!("{}", UpdateField::AvgFertilizingDays);
        let expected = "Average Fertilizing Days";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_prunnotes() {
        let result = format!("{}", UpdateField::PruningNotes);
        let expected = "Pruning Notes";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_companions() {
        let result = format!("{}", UpdateField::Companions);
        let expected = "Companions";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_notes() {
        let result = format!("{}", UpdateField::AdditionalNotes);
        let expected = "Additional Notes";
        assert_eq!(result, expected)
    }

    #[test]
    fn from_scientificname() {
        let result = UpdateField::from_str("scientific name").unwrap();
        let expected = UpdateField::ScientificName;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_genus() {
        let result = UpdateField::from_str("genus").unwrap();
        let expected = UpdateField::Genus;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_family() {
        let result = UpdateField::from_str("family").unwrap();
        let expected = UpdateField::Family;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_sun() {
        let result = UpdateField::from_str("sunlight").unwrap();
        let expected = UpdateField::Sunlight;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_mintemp() {
        let result = UpdateField::from_str("min temp").unwrap();
        let expected = UpdateField::TempMin;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_maxtemp() {
        let result = UpdateField::from_str("max temp").unwrap();
        let expected = UpdateField::TempMax;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_optmintemp() {
        let result = UpdateField::from_str("min temp opt").unwrap();
        let expected = UpdateField::TempMinOpt;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_optmaxtemp() {
        let result = UpdateField::from_str("max temp opt").unwrap();
        let expected = UpdateField::TempMaxOpt;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_plantdist() {
        let result = UpdateField::from_str("planting distance").unwrap();
        let expected = UpdateField::PlantingDistance;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_phmin() {
        let result = UpdateField::from_str("ph min").unwrap();
        let expected = UpdateField::PhMin;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_phmax() {
        let result = UpdateField::from_str("ph max").unwrap();
        let expected = UpdateField::PhMax;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_waternotes() {
        let result = UpdateField::from_str("watering notes").unwrap();
        let expected = UpdateField::WateringNotes;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_waterdays() {
        let result = UpdateField::from_str("average watering days").unwrap();
        let expected = UpdateField::AvgWateringDays;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_fertnotes() {
        let result = UpdateField::from_str("fertilizing notes").unwrap();
        let expected = UpdateField::FertilizingNotes;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_fertdays() {
        let result = UpdateField::from_str("average fertilizing days").unwrap();
        let expected = UpdateField::AvgFertilizingDays;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_prunenotes() {
        let result = UpdateField::from_str("pruning notes").unwrap();
        let expected = UpdateField::PruningNotes;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_companions() {
        let result = UpdateField::from_str("companions").unwrap();
        let expected = UpdateField::Companions;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_notes() {
        let result = UpdateField::from_str("additional notes").unwrap();
        let expected = UpdateField::AdditionalNotes;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_fail() {
        let result = UpdateField::from_str("other");
        assert!(result.is_err())
    }

    #[test]
    fn update_scientific_name() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::ScientificName,
            UpdateValue::Str("new name".to_owned()),
        )
        .unwrap();
        let mut expected = example_species();
        expected.scientific_name = "new name".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn update_genus() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::Genus,
            UpdateValue::Str("new genus".to_owned()),
        )
        .unwrap();
        let mut expected = example_species();
        expected.genus = "new genus".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn update_family() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::Family,
            UpdateValue::Str("new family".to_owned()),
        )
        .unwrap();
        let mut expected = example_species();
        expected.family = "new family".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn update_str_err() {
        let result = update_species(
            &mut example_species(),
            UpdateField::Family,
            UpdateValue::Fl(1.0),
        );
        assert!(result.is_err())
    }

    #[test]
    fn update_sun() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::Sunlight,
            UpdateValue::Sun(SunlightRequirement::Shade),
        )
        .unwrap();
        let mut expected = example_species();
        expected.sunlight = SunlightRequirement::Shade;
        assert_eq!(result, expected)
    }

    #[test]
    fn update_sun_err() {
        let result = update_species(
            &mut example_species(),
            UpdateField::Sunlight,
            UpdateValue::Fl(1.0),
        );
        assert!(result.is_err())
    }

    #[test]
    fn update_tempmin() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::TempMin,
            UpdateValue::Fl(1.0.to_owned()),
        )
        .unwrap();
        let mut expected = example_species();
        expected.temp_min = 1.0;
        assert_eq!(result, expected)
    }

    #[test]
    fn update_tempmax() {
        let mut result = example_species();
        update_species(&mut result, UpdateField::TempMax, UpdateValue::Fl(1.0)).unwrap();
        let mut expected = example_species();
        expected.temp_max = 1.0;
        assert_eq!(result, expected)
    }

    #[test]
    fn update_opttempmin() {
        let mut result = example_species();
        update_species(&mut result, UpdateField::TempMinOpt, UpdateValue::Fl(1.0)).unwrap();
        let mut expected = example_species();
        expected.opt_temp_min = 1.0;
        assert_eq!(result, expected)
    }

    #[test]
    fn update_opttempmpax() {
        let mut result = example_species();
        update_species(&mut result, UpdateField::TempMaxOpt, UpdateValue::Fl(1.0)).unwrap();
        let mut expected = example_species();
        expected.opt_temp_max = 1.0;
        assert_eq!(result, expected)
    }

    #[test]
    fn update_phmin() {
        let mut result = example_species();
        update_species(&mut result, UpdateField::PhMin, UpdateValue::Fl(1.0)).unwrap();
        let mut expected = example_species();
        expected.ph_min = 1.0;
        assert_eq!(result, expected)
    }

    #[test]
    fn update_phmax() {
        let mut result = example_species();
        update_species(&mut result, UpdateField::PhMax, UpdateValue::Fl(1.0)).unwrap();
        let mut expected = example_species();
        expected.ph_max = 1.0;
        assert_eq!(result, expected)
    }

    #[test]
    fn update_fl_err() {
        let result = update_species(
            &mut example_species(),
            UpdateField::PhMax,
            UpdateValue::Str("not a float".to_owned()),
        );
        assert!(result.is_err())
    }

    #[test]
    fn udpate_plantdist() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::PlantingDistance,
            UpdateValue::MFl(Some(1.0)),
        )
        .unwrap();
        let mut expected = example_species();
        expected.planting_distance = Some(1.0);
        assert_eq!(result, expected)
    }

    #[test]
    fn update_mfl_err() {
        let result = update_species(
            &mut example_species(),
            UpdateField::PlantingDistance,
            UpdateValue::Fl(1.0),
        );
        assert!(result.is_err())
    }

    #[test]
    fn update_waternotes_append() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::WateringNotes,
            UpdateValue::Note(vec!["note".to_owned()], true),
        )
        .unwrap();
        let mut expected = example_species();
        expected.watering_notes.push("note".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn update_waternotes_replace() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::WateringNotes,
            UpdateValue::Note(vec!["note".to_owned()], false),
        )
        .unwrap();
        let mut expected = example_species();
        expected.watering_notes = vec!["note".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn update_fertnotes_append() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::FertilizingNotes,
            UpdateValue::Note(vec!["note".to_owned()], true),
        )
        .unwrap();
        let mut expected = example_species();
        expected.fertilizing_notes.push("note".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn update_fertnotes_replace() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::FertilizingNotes,
            UpdateValue::Note(vec!["note".to_owned()], false),
        )
        .unwrap();
        let mut expected = example_species();
        expected.fertilizing_notes = vec!["note".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn update_prunenotes_append() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::PruningNotes,
            UpdateValue::Note(vec!["note".to_owned()], true),
        )
        .unwrap();
        let mut expected = example_species();
        expected.pruning_notes.push("note".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn update_prunenotes_replace() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::PruningNotes,
            UpdateValue::Note(vec!["note".to_owned()], false),
        )
        .unwrap();
        let mut expected = example_species();
        expected.pruning_notes = vec!["note".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn update_companions_append() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::Companions,
            UpdateValue::Note(vec!["note".to_owned()], true),
        )
        .unwrap();
        let mut expected = example_species();
        expected.companions.push("note".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn update_companions_replace() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::Companions,
            UpdateValue::Note(vec!["note".to_owned()], false),
        )
        .unwrap();
        let mut expected = example_species();
        expected.companions = vec!["note".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn update_notes_append() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::AdditionalNotes,
            UpdateValue::Note(vec!["note".to_owned()], true),
        )
        .unwrap();
        let mut expected = example_species();
        expected.additional_notes.push("note".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn update_notes_replace() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::AdditionalNotes,
            UpdateValue::Note(vec!["note".to_owned()], false),
        )
        .unwrap();
        let mut expected = example_species();
        expected.additional_notes = vec!["note".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn update_notes_err() {
        let result = update_species(
            &mut example_species(),
            UpdateField::AdditionalNotes,
            UpdateValue::Fl(1.0),
        );
        assert!(result.is_err())
    }

    #[test]
    fn update_waterdays() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::AvgWateringDays,
            UpdateValue::MInt(Some(1)),
        )
        .unwrap();
        let mut expected = example_species();
        expected.avg_watering_days = Some(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn update_fertdays() {
        let mut result = example_species();
        update_species(
            &mut result,
            UpdateField::AvgFertilizingDays,
            UpdateValue::MInt(Some(2)),
        )
        .unwrap();
        let mut expected = example_species();
        expected.avg_fertilizing_days = Some(2);
        assert_eq!(result, expected)
    }

    #[test]
    fn update_mint_err() {
        let result = update_species(
            &mut example_species(),
            UpdateField::AvgFertilizingDays,
            UpdateValue::Fl(1.0),
        );
        assert!(result.is_err())
    }
}
