use super::{
    input_handlers::{input_plant_name, str_to_value},
    Action, BotAction,
};
use crate::errors::Error;
use database::database_manager::DatabaseManager;
use plants::plant_update::{update_plant, UpdateField, UpdateValue};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Step {
    PlantName,
    UpdateField,
    UpdateValue,
    Done,
}

#[derive(Debug, PartialEq, Clone)]
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
                println!("parsing value {input}");
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
            Step::PlantName => Ok("Please enter plant name".to_owned()),
            Step::UpdateField => Ok(format!(
                "Please enter field to update, possible fields: {}",
                UpdateField::fields_strs().join(", ")
            )),
            Step::UpdateValue => {
                Ok("Please enter updated value (notes will be appended)".to_owned())
            }
            Step::Done => Err(Error::ActionAlreadyDone("Update Species".to_owned())),
        }
    }
}

impl From<UpdatePlant> for BotAction {
    fn from(updpl: UpdatePlant) -> BotAction {
        BotAction::UpdatePlant(updpl)
    }
}

#[cfg(test)]
mod update_plant_tests {
    use super::{Action, BotAction, Step, UpdateField, UpdatePlant, UpdateValue};
    use crate::test_common::DummyManager;

    #[test]
    fn update_default() {
        let result = UpdatePlant::default();
        let expected = UpdatePlant {
            current_step: Step::PlantName,
            plant_name: None,
            update_field: None,
            update_value: None,
            date_format: "%d.%m.%Y".to_owned(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plant() {
        let mut result = UpdatePlant::default();
        result
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = UpdatePlant::default();
        expected.current_step = Step::UpdateField;
        expected.plant_name = Some("Plant1".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plant_err() {}

    #[test]
    fn input_field() {
        let mut result = UpdatePlant::default();
        result.current_step = Step::UpdateField;
        result
            .handle_input("auto watered".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = UpdatePlant::default();
        expected.current_step = Step::UpdateValue;
        expected.update_field = Some(UpdateField::AutoWater);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_field_err() {
        let mut action = UpdatePlant::default();
        action.current_step = Step::UpdateField;
        let result = action.handle_input("not a valid field".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_value() {
        let mut result = UpdatePlant::default();
        result.current_step = Step::UpdateValue;
        result.update_field = Some(UpdateField::AutoWater);
        result
            .handle_input("y".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = UpdatePlant::default();
        expected.current_step = Step::Done;
        expected.update_field = Some(UpdateField::AutoWater);
        expected.update_value = Some(UpdateValue::Bool(true));
        assert_eq!(result, expected)
    }

    #[test]
    fn input_value_err() {
        let mut action = UpdatePlant::default();
        action.current_step = Step::UpdateValue;
        action.update_field = Some(UpdateField::AutoWater);
        let result = action.handle_input("not a valid boolean".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_value_no_field() {
        let mut action = UpdatePlant::default();
        action.current_step = Step::UpdateValue;
        let result = action.handle_input("y".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_err() {
        let mut action = UpdatePlant::default();
        action.current_step = Step::Done;
        let result = action.handle_input("".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn done_done() {
        let mut action = UpdatePlant::default();
        action.current_step = Step::Done;
        assert!(action.is_done())
    }

    #[test]
    fn done_notdone() {
        assert!(!UpdatePlant::default().is_done())
    }

    #[test]
    fn write_no_plant() {
        let action = UpdatePlant::default();
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_field() {
        let mut action = UpdatePlant::default();
        action.plant_name = Some("Plant1".to_owned());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_value() {
        let mut action = UpdatePlant::default();
        action.plant_name = Some("Plant1".to_owned());
        action.update_field = Some(UpdateField::AutoWater);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write() {
        let mut action = UpdatePlant::default();
        action.plant_name = Some("Plant1".to_owned());
        action.update_field = Some(UpdateField::AutoWater);
        action.update_value = Some(UpdateValue::Bool(true));
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_ok())
    }

    #[test]
    fn next_plant() {
        let action = UpdatePlant::default();
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter plant name";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_field() {
        let mut action = UpdatePlant::default();
        action.current_step = Step::UpdateField;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter field to update, possible fields: Origin, Species, Location, Obtained, Notes, Auto Watered";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_value() {
        let mut action = UpdatePlant::default();
        action.current_step = Step::UpdateValue;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter updated value (notes will be appended)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_err() {
        let mut action = UpdatePlant::default();
        action.current_step = Step::Done;
        let result = action.get_next_prompt();
        assert!(result.is_err())
    }

    #[test]
    fn into_action() {
        let result = <UpdatePlant as Into<BotAction>>::into(UpdatePlant::default());
        let expected = BotAction::UpdatePlant(UpdatePlant::default());
        assert_eq!(result, expected)
    }
}
