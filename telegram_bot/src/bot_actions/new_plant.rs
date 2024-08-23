use super::{
    input_handlers::{input_health, input_species},
    Action, BotAction,
};
use crate::errors::Error;
use chrono::NaiveDate;
use database::database_manager::{DatabaseManager, PlantJSON};

#[derive(PartialEq, Eq)]
enum Step {
    PlantName,
    SpeciesName,
    Height,
    Width,
    Health,
    Location,
    AutoWatered,
    Origin,
    ObtainedDate,
    Notes,
    Done,
}

pub struct NewPlant {
    current_step: Step,
    date_format: String,
    plant_name: Option<String>,
    species_name: Option<String>,
    height: Option<f32>,
    width: Option<f32>,
    health: Option<i32>,
    location: Option<String>,
    autowatered: Option<bool>,
    origin: Option<String>,
    obtained: Option<NaiveDate>,
    notes: Option<Vec<String>>,
}

impl Action for NewPlant {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::PlantName => {
                let name = input.trim().to_owned();
                let exists = db_man.plant_exists(&name)?;
                if exists {
                    Err(Error::PlantExists(name))
                } else {
                    self.plant_name = Some(name);
                    self.current_step = Step::SpeciesName;
                    Ok(())
                }
            }
            Step::SpeciesName => {
                let name = input_species(input, db_man)?;
                self.species_name = Some(name);
                self.current_step = Step::Height;
                Ok(())
            }
            Step::Height => {
                let height = input
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Height".to_owned()))?;
                self.height = Some(height);
                self.current_step = Step::Width;
                Ok(())
            }
            Step::Width => {
                let width = input
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Width".to_owned()))?;
                self.width = Some(width);
                self.current_step = Step::Health;
                todo!("")
            }
            Step::Health => {
                let health = input_health(input)?;
                self.health = Some(health);
                self.current_step = Step::Location;
                Ok(())
            }
            Step::Location => {
                self.location = Some(input.trim().to_owned());
                self.current_step = Step::AutoWatered;
                Ok(())
            }
            Step::AutoWatered => {
                let is_autowatered = input.trim().to_lowercase() == "y";
                self.autowatered = Some(is_autowatered);
                self.current_step = Step::Origin;
                Ok(())
            }
            Step::Origin => {
                self.origin = Some(input.trim().to_owned());
                self.current_step = Step::ObtainedDate;
                Ok(())
            }
            Step::ObtainedDate => {
                let date = NaiveDate::parse_from_str(input.trim(), &self.date_format)
                    .map_err(|_| Error::ParseError("Date".to_owned()))?;
                self.obtained = Some(date);
                self.current_step = Step::Notes;
                Ok(())
            }
            Step::Notes => {
                let notes: Vec<String> = input.split(',').map(|st| st.trim().to_owned()).collect();
                self.notes = Some(notes);
                self.current_step = Step::Done;
                Ok(())
            }
            Step::Done => Err(Error::ActionAlreadyDone("New Plant".to_owned())),
        }
    }

    fn is_done(&self) -> bool {
        self.current_step == Step::Done
    }

    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let plant_name = self
            .plant_name
            .clone()
            .ok_or(Error::MissingInput("Plant Name".to_owned()))?;
        let auto_watering = self
            .autowatered
            .ok_or(Error::MissingInput("Autowatered".to_owned()))?;
        let current_location = self
            .location
            .clone()
            .ok_or(Error::MissingInput("Location".to_owned()))?;
        let obtained = self
            .obtained
            .ok_or(Error::MissingInput("Obtained Date".to_owned()))?;
        let origin = self
            .origin
            .clone()
            .ok_or(Error::MissingInput("Origin".to_owned()))?;
        let plant_health = self
            .health
            .ok_or(Error::MissingInput("Health".to_owned()))?;
        let plant_notes = self
            .notes
            .clone()
            .ok_or(Error::MissingInput("Notes".to_owned()))?;
        let species_name = self
            .species_name
            .clone()
            .ok_or(Error::MissingInput("Species Name".to_owned()))?;

        let plant_json = PlantJSON {
            plant_name: plant_name.clone(),
            auto_watering: auto_watering.into(),
            current_location,
            obtained: obtained.format(&self.date_format).to_string(),
            origin,
            plant_health: plant_health.to_string(),
            plant_notes,
            species_name,
        };

        db_man.write_plant(plant_json)?;
        let ret_msg = format!("Successfully saved plant {plant_name}");
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::PlantName => Ok("Please enter plant name".to_owned()),
            Step::SpeciesName => Ok("Please enter species name".to_owned()),
            Step::Height => Ok("Please enter height (cm)".to_owned()),
            Step::Width => Ok("Please enter width (cm)".to_owned()),
            Step::Health => Ok("Please enter health (0-5)".to_owned()),
            Step::Location => Ok("Please enter location".to_owned()),
            Step::AutoWatered => Ok("Is plant autowatered (y/n)".to_owned()),
            Step::Origin => Ok("Please enter plant origin".to_owned()),
            Step::ObtainedDate => Ok("Please enter obtained date".to_owned()),
            Step::Notes => Ok("Please enter notes (Enter \"Done\" for no notes)".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("New Plant".to_owned())),
        }
    }
}

impl From<NewPlant> for BotAction {
    fn from(newp: NewPlant) -> BotAction {
        BotAction::NewPlant(newp)
    }
}
