use super::{
    input_handlers::{input_plant_name, str_to_value},
    Action, BotAction,
};
use crate::errors::Error;
use database::database_manager::DatabaseManager;
use plants::plant_update::{update_plant, UpdateField, UpdateValue};

#[derive(PartialEq, Eq, Clone)]
enum Step {
    PlantName,
    UpdateField,
    UpdateValue,
    Done,
}

#[derive(Clone)]
pub struct UpdatePlant {
    current_step: Step,
    plant_name: Option<String>,
    update_field: Option<UpdateField>,
    update_value: Option<UpdateValue>,
    date_format: String,
}

impl UpdatePlant {
    pub fn new(date_format: &str) -> UpdatePlant {
        UpdatePlant {
            current_step: Step::PlantName,
            plant_name: None,
            update_field: None,
            update_value: None,
            date_format: date_format.to_owned(),
        }
    }
}

impl Default for UpdatePlant {
    fn default() -> Self {
        UpdatePlant::new("%d.%m.%Y")
    }
}

impl Action for UpdatePlant {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::PlantName => {
                let name = input_plant_name(input, db_man)?;
                self.plant_name = Some(name);
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
                let value = str_to_value(input, field, db_man, &self.date_format)?;
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
        let plant_name = self
            .plant_name
            .clone()
            .ok_or(Error::MissingInput("Species Name".to_owned()))?;
        let mut plant = db_man.get_plant(&plant_name)?;
        let update_field = self
            .update_field
            .clone()
            .ok_or(Error::MissingInput("Update Field".to_owned()))?;
        let update_value = self
            .update_value
            .clone()
            .ok_or(Error::MissingInput("Update Value".to_owned()))?;
        update_plant(&mut plant, update_field, update_value)?;
        db_man.write_plant(plant.info)?;
        let ret_msg = format!("Successfully updated plant {plant_name}");
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::PlantName => Ok("Please enter Species name".to_owned()),
            Step::UpdateField => Ok(format!(
                "Please enter field to update, possible fields: {}",
                UpdateField::fields_strs().join(", ")
            )),
            Step::UpdateValue => Ok("Enter updated value (notes will be appended".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("Update Species".to_owned())),
        }
    }
}

impl From<UpdatePlant> for BotAction {
    fn from(updpl: UpdatePlant) -> BotAction {
        BotAction::UpdatePlant(updpl)
    }
}
