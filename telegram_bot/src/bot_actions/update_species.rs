use super::{input_handlers::input_species, Action, BotAction};
use crate::errors::Error;
use database::database_manager::DatabaseManager;
use plants::species_update::{update_species, UpdateField, UpdateValue};

#[derive(PartialEq, Eq, Clone)]
enum Step {
    SpeciesName,
    UpdateField,
    UpdateValue,
    Done,
}

#[derive(Clone)]
pub struct UpdateSpecies {
    current_step: Step,
    species_name: Option<String>,
    update_field: Option<UpdateField>,
    update_value: Option<UpdateValue>,
}

impl UpdateSpecies {
    pub fn new() -> UpdateSpecies {
        UpdateSpecies {
            current_step: Step::SpeciesName,
            species_name: None,
            update_field: None,
            update_value: None,
        }
    }
}

impl Default for UpdateSpecies {
    fn default() -> Self {
        UpdateSpecies::new()
    }
}

impl Action for UpdateSpecies {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::SpeciesName => {
                let name = input_species(input, db_man)?;
                self.species_name = Some(name);
                self.current_step = Step::UpdateField;
                Ok(())
            }
            Step::UpdateField => {
                let field = input
                    .parse::<UpdateField>()
                    .map_err(|_| Error::ParseError("Update Field".to_owned()))?;
                self.update_field = Some(field);
                self.current_step = Step::UpdateValue;
                Ok(())
            }
            Step::UpdateValue => {
                let field = self
                    .update_field
                    .as_ref()
                    .ok_or(Error::MissingInput("Update Field".to_owned()))?;
                let value = (input, field).try_into()?;
                self.update_value = Some(value);
                self.current_step = Step::Done;
                Ok(())
            }
            Step::Done => Err(Error::ActionAlreadyDone("Update Species".to_owned())),
        }
    }

    fn is_done(&self) -> bool {
        self.current_step == Step::Done
    }

    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let species_name = self
            .species_name
            .clone()
            .ok_or(Error::MissingInput("Species Name".to_owned()))?;
        let mut species = db_man.get_species(&species_name)?;
        let update_field = self
            .update_field
            .clone()
            .ok_or(Error::MissingInput("Update Field".to_owned()))?;
        let update_value = self
            .update_value
            .clone()
            .ok_or(Error::MissingInput("Update Value".to_owned()))?;
        update_species(&mut species, update_field, update_value)?;
        db_man.write_species(species)?;
        let ret_msg = format!("Successfully updated species {species_name}");
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::SpeciesName => Ok("Please enter Species name".to_owned()),
            Step::UpdateField => Ok(format!(
                "Please enter field to update, possible fields: {}",
                UpdateField::fields_strs().join(", ")
            )),
            Step::UpdateValue => Ok("Enter updated value (notes will be appended".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("Update Species".to_owned())),
        }
    }
}

impl From<UpdateSpecies> for BotAction {
    fn from(updsp: UpdateSpecies) -> BotAction {
        BotAction::UpdateSpecies(updsp)
    }
}
