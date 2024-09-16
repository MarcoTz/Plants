use super::{input_handlers::input_species, Action, BotAction};
use crate::errors::Error;
use database::database_manager::DatabaseManager;
use plants::species_update::{update_species, UpdateField, UpdateValue};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Step {
    SpeciesName,
    UpdateField,
    UpdateValue,
    Done,
}

#[derive(Debug, PartialEq, Clone)]
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
            Step::SpeciesName => Ok("Please enter Species".to_owned()),
            Step::UpdateField => Ok(format!(
                "Please enter field to update, possible fields: {}",
                UpdateField::fields_strs().join(", ")
            )),
            Step::UpdateValue => Ok("Please enter new value (notes will be appended".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("Update Species".to_owned())),
        }
    }
}

impl From<UpdateSpecies> for BotAction {
    fn from(updsp: UpdateSpecies) -> BotAction {
        BotAction::UpdateSpecies(updsp)
    }
}

#[cfg(test)]
mod update_species_tests {

    use super::{Action, BotAction, Step, UpdateField, UpdateSpecies, UpdateValue};
    use crate::test_common::DummyManager;

    #[test]
    fn update_default() {
        let result = UpdateSpecies::default();
        let expected = UpdateSpecies {
            current_step: Step::SpeciesName,
            species_name: None,
            update_field: None,
            update_value: None,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn input_species() {
        let mut result = UpdateSpecies::default();
        result
            .handle_input("Species1".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = UpdateSpecies::default();
        expected.current_step = Step::UpdateField;
        expected.species_name = Some("Species1".to_owned());
    }

    #[test]
    fn input_species_err() {
        let result =
            UpdateSpecies::default().handle_input("not a species".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_field() {
        let mut result = UpdateSpecies::default();
        result.current_step = Step::UpdateField;
        result
            .handle_input("Genus".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = UpdateSpecies::default();
        expected.current_step = Step::UpdateValue;
        expected.update_field = Some(UpdateField::Genus);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_field_err() {
        let mut action = UpdateSpecies::default();
        action.current_step = Step::UpdateField;
        let result = action.handle_input("not a valid field".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_value() {
        let mut result = UpdateSpecies::default();
        result.current_step = Step::UpdateValue;
        result.update_field = Some(UpdateField::Genus);
        result
            .handle_input("NewGenus".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = UpdateSpecies::default();
        expected.current_step = Step::Done;
        expected.update_field = Some(UpdateField::Genus);
        expected.update_value = Some(UpdateValue::Str("NewGenus".to_owned()));
        assert_eq!(result, expected)
    }

    #[test]
    fn input_value_err() {
        let mut action = UpdateSpecies::default();
        action.current_step = Step::UpdateValue;
        action.update_field = Some(UpdateField::TempMin);
        let result = action.handle_input("not a number".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_value_no_field() {
        let mut action = UpdateSpecies::default();
        action.current_step = Step::UpdateValue;
        let result = action.handle_input("".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }
    #[test]
    fn input_err() {
        let mut action = UpdateSpecies::default();
        action.current_step = Step::Done;
        let result = action.handle_input("".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn done_done() {
        let mut action = UpdateSpecies::default();
        action.current_step = Step::Done;
        assert!(action.is_done())
    }

    #[test]
    fn done_notdone() {
        assert!(!UpdateSpecies::default().is_done())
    }

    #[test]
    fn write_no_species() {
        let result = UpdateSpecies::default().write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_field() {
        let mut action = UpdateSpecies::default();
        action.species_name = Some("Species1".to_owned());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_value() {
        let mut action = UpdateSpecies::default();
        action.species_name = Some("Species1".to_owned());
        action.update_field = Some(UpdateField::Genus);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write() {
        let mut action = UpdateSpecies::default();
        action.species_name = Some("Species1".to_owned());
        action.update_field = Some(UpdateField::Genus);
        action.update_value = Some(UpdateValue::Str("NewGenus".to_owned()));
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_ok())
    }

    #[test]
    fn next_species() {
        let result = UpdateSpecies::default().get_next_prompt().unwrap();
        let expected = "Please enter Species";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_field() {
        let mut action = UpdateSpecies::default();
        action.current_step = Step::UpdateField;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter field to update, possible fields: Scientific Name, Genus, Family, Sunlight, Min Temp, Max Temp, Min Temp Opt, Max Temp Opt, pH Min, pH Max, Planting Distance, Watering Notes, Fertilizing Notes, Pruning Notes, Companions, Additional Notes, Average Watering Days, Average Fertilizing Days";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_value() {
        let mut action = UpdateSpecies::default();
        action.current_step = Step::UpdateValue;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter new value (notes will be appended";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_err() {
        let mut action = UpdateSpecies::default();
        action.current_step = Step::Done;
        let result = action.get_next_prompt();
        assert!(result.is_err())
    }

    #[test]
    fn into_action() {
        let result = <UpdateSpecies as Into<BotAction>>::into(UpdateSpecies::default());
        let expected = BotAction::UpdateSpecies(UpdateSpecies::default());
        assert_eq!(result, expected)
    }
}
