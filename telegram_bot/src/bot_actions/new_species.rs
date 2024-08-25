use super::{input_handlers::input_notes, Action, BotAction};
use crate::errors::Error;
use database::database_manager::DatabaseManager;
use plants::species::{Species, SunlightRequirement};

#[derive(PartialEq, Eq)]
enum Step {
    SpeciesName,
    ScientificName,
    Genus,
    Family,
    Sunlight,
    MinTemp,
    MaxTemp,
    MinTempOpt,
    MaxTempOpt,
    PlantDist,
    PhMin,
    PhMax,
    AvgWateringDays,
    WateringNotes,
    AvgFertilizingDays,
    FertilizingNotes,
    PruningNotes,
    CompanionPlants,
    Notes,
    Done,
}
pub struct NewSpecies {
    current_step: Step,
    species_name: Option<String>,
    scientific_name: Option<String>,
    genus: Option<String>,
    family: Option<String>,
    sunlight: Option<SunlightRequirement>,
    min_temp: Option<f32>,
    max_temp: Option<f32>,
    min_temp_opt: Option<f32>,
    max_temp_opt: Option<f32>,
    planting_distance: Option<f32>,
    ph_min: Option<f32>,
    ph_max: Option<f32>,
    avg_watering: Option<i32>,
    watering_notes: Option<Vec<String>>,
    avg_fertilizing: Option<i32>,
    fertilizing_notes: Option<Vec<String>>,
    pruning_notes: Option<Vec<String>>,
    companions: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl NewSpecies {
    pub fn new() -> NewSpecies {
        NewSpecies {
            current_step: Step::SpeciesName,
            species_name: None,
            scientific_name: None,
            genus: None,
            family: None,
            sunlight: None,
            min_temp: None,
            max_temp: None,
            min_temp_opt: None,
            max_temp_opt: None,
            planting_distance: None,
            ph_min: None,
            ph_max: None,
            avg_watering: None,
            watering_notes: None,
            avg_fertilizing: None,
            fertilizing_notes: None,
            pruning_notes: None,
            companions: None,
            notes: None,
        }
    }
}

impl Action for NewSpecies {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::SpeciesName => {
                let name = input.trim().to_owned();
                let exists = db_man.species_exists(&name)?;
                if exists {
                    Err(Error::SpeciesExists(name))
                } else {
                    self.species_name = Some(name);
                    self.current_step = Step::ScientificName;
                    Ok(())
                }
            }
            Step::ScientificName => {
                let name = input.trim().to_owned();
                self.scientific_name = Some(name);
                self.current_step = Step::Genus;
                Ok(())
            }
            Step::Genus => {
                let genus = input.trim().to_owned();
                self.genus = Some(genus);
                self.current_step = Step::Family;
                Ok(())
            }
            Step::Family => {
                let family = input.trim().to_owned();
                self.family = Some(family);
                self.current_step = Step::Sunlight;
                Ok(())
            }
            Step::Sunlight => {
                let sunlight = input
                    .trim()
                    .to_lowercase()
                    .parse::<SunlightRequirement>()
                    .map_err(|_| Error::ParseError("Sunlight".to_owned()))?;
                self.sunlight = Some(sunlight);
                self.current_step = Step::MinTemp;
                Ok(())
            }
            Step::MinTemp => {
                let temp = input
                    .trim()
                    .to_lowercase()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Temperature".to_owned()))?;
                self.min_temp = Some(temp);
                self.current_step = Step::MaxTemp;
                Ok(())
            }
            Step::MaxTemp => {
                let temp = input
                    .trim()
                    .to_lowercase()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Temperature".to_owned()))?;
                self.max_temp = Some(temp);
                self.current_step = Step::MinTempOpt;
                Ok(())
            }
            Step::MinTempOpt => {
                let temp = input
                    .trim()
                    .to_lowercase()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Temperature".to_owned()))?;
                self.min_temp_opt = Some(temp);
                self.current_step = Step::MaxTempOpt;
                Ok(())
            }
            Step::MaxTempOpt => {
                let temp = input
                    .trim()
                    .to_lowercase()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Temperature".to_owned()))?;
                self.max_temp_opt = Some(temp);
                self.current_step = Step::PlantDist;
                Ok(())
            }
            Step::PlantDist => {
                let dist = input
                    .trim()
                    .to_lowercase()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Planting Distance".to_owned()))?;
                self.planting_distance = if dist < 0.0 { None } else { Some(dist) };
                self.current_step = Step::PhMin;
                Ok(())
            }
            Step::PhMin => {
                let ph = input
                    .trim()
                    .to_lowercase()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Ph Value".to_owned()))?;
                self.ph_min = Some(ph);
                self.current_step = Step::PhMax;
                Ok(())
            }
            Step::PhMax => {
                let ph = input
                    .trim()
                    .to_lowercase()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Ph Value".to_owned()))?;
                self.ph_max = Some(ph);
                self.current_step = Step::AvgWateringDays;
                Ok(())
            }
            Step::AvgWateringDays => {
                let days = input
                    .trim()
                    .to_lowercase()
                    .parse::<i32>()
                    .map_err(|_| Error::ParseError("Watering Days".to_owned()))?;
                self.avg_watering = if days < 0 { None } else { Some(days) };
                self.current_step = Step::WateringNotes;
                Ok(())
            }
            Step::WateringNotes => {
                let notes = input_notes(input);
                self.watering_notes = Some(notes);
                self.current_step = Step::AvgFertilizingDays;
                Ok(())
            }
            Step::AvgFertilizingDays => {
                let days = input
                    .trim()
                    .to_lowercase()
                    .parse::<i32>()
                    .map_err(|_| Error::ParseError("Fertilizing Days".to_owned()))?;
                self.avg_fertilizing = if days < 0 { None } else { Some(days) };
                self.current_step = Step::FertilizingNotes;
                Ok(())
            }
            Step::FertilizingNotes => {
                let notes = input_notes(input);
                self.fertilizing_notes = Some(notes);
                self.current_step = Step::PruningNotes;
                Ok(())
            }
            Step::PruningNotes => {
                let notes = input_notes(input);
                self.pruning_notes = Some(notes);
                self.current_step = Step::CompanionPlants;
                Ok(())
            }
            Step::CompanionPlants => {
                let notes = input_notes(input);
                self.companions = Some(notes);
                self.current_step = Step::Notes;
                Ok(())
            }
            Step::Notes => {
                let notes = input_notes(input);
                self.notes = Some(notes);
                self.current_step = Step::Done;
                Ok(())
            }
            Step::Done => Err(Error::ActionAlreadyDone("New Species".to_owned())),
        }
    }

    fn is_done(&self) -> bool {
        self.current_step == Step::Done
    }
    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let name = self
            .species_name
            .clone()
            .ok_or(Error::MissingInput("Species name".to_owned()))?;
        let scientific_name = self
            .scientific_name
            .clone()
            .ok_or(Error::MissingInput("Scientific Name".to_owned()))?;
        let genus = self
            .genus
            .clone()
            .ok_or(Error::MissingInput("Genus".to_owned()))?;
        let family = self
            .family
            .clone()
            .ok_or(Error::MissingInput("Family".to_owned()))?;
        let sunlight = self
            .sunlight
            .clone()
            .ok_or(Error::MissingInput("Sunlight".to_owned()))?;
        let temp_min = self
            .min_temp
            .ok_or(Error::MissingInput("Minimal Temperature".to_owned()))?;
        let temp_max = self
            .max_temp
            .ok_or(Error::MissingInput("Maximal Temperature".to_owned()))?;
        let opt_temp_min = self.min_temp_opt.ok_or(Error::MissingInput(
            "Minimal Optimal Temperature".to_owned(),
        ))?;
        let opt_temp_max = self.max_temp_opt.ok_or(Error::MissingInput(
            "Maximal Optimal Temperature".to_owned(),
        ))?;
        let ph_min = self
            .ph_min
            .ok_or(Error::MissingInput("Minimum pH".to_owned()))?;
        let ph_max = self
            .ph_max
            .ok_or(Error::MissingInput("Maximum pH".to_owned()))?;
        let watering_notes = self
            .watering_notes
            .clone()
            .ok_or(Error::MissingInput("Watering Notes".to_owned()))?;
        let fertilizing_notes = self
            .fertilizing_notes
            .clone()
            .ok_or(Error::MissingInput("Fertilizing Notes".to_owned()))?;
        let pruning_notes = self
            .pruning_notes
            .clone()
            .ok_or(Error::MissingInput("Pruning Notes".to_owned()))?;
        let companions = self
            .companions
            .clone()
            .ok_or(Error::MissingInput("Companions".to_owned()))?;
        let additional_notes = self
            .notes
            .clone()
            .ok_or(Error::MissingInput("Additional Notes".to_owned()))?;
        let species = Species {
            name: name.clone(),
            scientific_name,
            genus,
            family,
            sunlight,
            temp_min,
            temp_max,
            opt_temp_min,
            opt_temp_max,
            planting_distance: self.planting_distance,
            ph_min,
            ph_max,
            watering_notes,
            avg_watering_days: self.avg_watering,
            fertilizing_notes,
            avg_fertilizing_days: self.avg_fertilizing,
            pruning_notes,
            companions,
            additional_notes,
        };
        db_man.write_species(species)?;
        let ret_msg = format!("Successfully created species {name}");
        Ok(ret_msg)
    }
    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::SpeciesName => Ok("Please enter (common) name".to_owned()),
            Step::ScientificName => Ok("Please enter scientific name".to_owned()),
            Step::Genus => Ok("Please enter species genus".to_owned()),
            Step::Family => Ok("Please enter species family".to_owned()),
            Step::Sunlight => {
                Ok("Please enter sunlight requirements (direct/indirect/shade)".to_owned())
            }
            Step::MinTemp => Ok("Please enter minimal (survivable) temperature".to_owned()),
            Step::MaxTemp => Ok("Please enter maximal (survivable) temperature".to_owned()),
            Step::MinTempOpt => Ok("Please enter minimal (optimal) temperature".to_owned()),
            Step::MaxTempOpt => Ok("Please enter maximal (optimal) temperature".to_owned()),
            Step::PlantDist => Ok(
                "Please enter mimimal distance for rows of seeds (- if not applicable)".to_owned(),
            ),
            Step::PhMin => Ok("Please enter minimal pH value".to_owned()),
            Step::PhMax => Ok("Please enter maximal pH value".to_owned()),
            Step::AvgWateringDays => Ok(
                "Please enter average number of days between waterings (-1 if not applicable)"
                    .to_owned(),
            ),
            Step::WateringNotes => Ok(
                "Please enter watering notes (separate by comma, \"Done\" for no notes)".to_owned(),
            ),
            Step::AvgFertilizingDays => Ok(
                "Please enter average days between fertilizings (-1 if not applcable)".to_owned(),
            ),
            Step::FertilizingNotes => Ok(
                "Please enter fertilizing notes (separate by comma, \"Done\" for no notes)"
                    .to_owned(),
            ),
            Step::PruningNotes => Ok(
                "Please enter pruning notes (separate by comma, \"Done\" for no notes)".to_owned(),
            ),
            Step::CompanionPlants => Ok(
                "Please enter companion plants (separate by comma, \"Done\" for no notes)"
                    .to_owned(),
            ),
            Step::Notes => Ok(
                "Please enter any additional notes (separate by comma, \"Done\" for no notes)"
                    .to_owned(),
            ),
            Step::Done => Err(Error::ActionAlreadyDone("New Species".to_owned())),
        }
    }
}

impl From<NewSpecies> for BotAction {
    fn from(newsp: NewSpecies) -> BotAction {
        BotAction::NewSpecies(newsp)
    }
}
