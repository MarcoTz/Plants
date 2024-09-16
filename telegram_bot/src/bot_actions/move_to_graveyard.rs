use super::{input_handlers::input_plant_name, Action, BotAction};
use crate::errors::Error;
use chrono::NaiveDate;
use database::database_manager::DatabaseManager;
use plants::{graveyard::GraveyardPlant, named::Named};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Step {
    PlantName,
    DiedDate,
    Reason,
    Done,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoveToGraveyard {
    current_step: Step,
    date_format: String,
    plant_name: Option<String>,
    died_date: Option<NaiveDate>,
    reason: Option<String>,
}

impl MoveToGraveyard {
    pub fn new(date_format: &str) -> MoveToGraveyard {
        MoveToGraveyard {
            current_step: Step::PlantName,
            date_format: date_format.to_owned(),
            plant_name: None,
            died_date: None,
            reason: None,
        }
    }
}

impl Default for MoveToGraveyard {
    fn default() -> Self {
        MoveToGraveyard::new("%d.%m.%Y")
    }
}

impl Action for MoveToGraveyard {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::PlantName => {
                let name = input_plant_name(input, db_man)?;
                self.plant_name = Some(name);
                self.current_step = Step::DiedDate;
                Ok(())
            }
            Step::DiedDate => {
                let date = NaiveDate::parse_from_str(input.trim(), &self.date_format)
                    .map_err(|_| Error::ParseError("Died Date".to_owned()))?;
                self.died_date = Some(date);
                self.current_step = Step::Reason;
                Ok(())
            }
            Step::Reason => {
                let reason = input.trim().to_owned();
                self.reason = Some(reason);
                self.current_step = Step::Done;
                Ok(())
            }
            Step::Done => Err(Error::ActionAlreadyDone("Move To Graveyard".to_owned())),
        }
    }
    fn is_done(&self) -> bool {
        self.current_step == Step::Done
    }
    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let name = self
            .plant_name
            .clone()
            .ok_or(Error::MissingInput("Plant Name".to_owned()))?;
        let died = self
            .died_date
            .ok_or(Error::MissingInput("Died Date".to_owned()))?;
        let reason = self
            .reason
            .clone()
            .ok_or(Error::MissingInput("Died Reason".to_owned()))?;
        let plant = db_man.get_plant(&name)?;
        let gr_plant = GraveyardPlant {
            name: name.clone(),
            species: plant.info.species.get_name(),
            planted: plant.info.obtained,
            died,
            reason,
        };
        db_man.kill_plant(gr_plant)?;
        let ret_msg = format!("Successfully moved plant {name} to graveyard");
        Ok(ret_msg)
    }
    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::PlantName => Ok("Please enter Plant Name".to_owned()),
            Step::DiedDate => Ok("Please enter Died Date".to_owned()),
            Step::Reason => Ok("Please enter Died Reason".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("Move To Graveyard".to_owned())),
        }
    }
}

impl From<MoveToGraveyard> for BotAction {
    fn from(gr: MoveToGraveyard) -> BotAction {
        BotAction::MoveToGraveyard(gr)
    }
}

#[cfg(test)]
mod move_graveyard_tests {
    use super::{Action, BotAction, MoveToGraveyard, Step};
    use crate::test_common::{example_date1, DummyManager};

    #[test]
    fn move_graveyard_default() {
        let result = MoveToGraveyard::default();
        let expected = MoveToGraveyard {
            current_step: Step::PlantName,
            date_format: "%d.%m.%Y".to_owned(),
            plant_name: None,
            died_date: None,
            reason: None,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plant() {
        let mut result = MoveToGraveyard::default();
        result
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = MoveToGraveyard::default();
        expected.current_step = Step::DiedDate;
        expected.plant_name = Some("Plant1".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plant_err() {
        let result =
            MoveToGraveyard::default().handle_input("not a plant".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_died() {
        let mut result = MoveToGraveyard::default();
        result.current_step = Step::DiedDate;
        result
            .handle_input("01.01.1970".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = MoveToGraveyard::default();
        expected.current_step = Step::Reason;
        expected.died_date = Some(example_date1());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_died_err() {
        let mut action = MoveToGraveyard::default();
        action.current_step = Step::DiedDate;
        let result = action.handle_input("not a date".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_reason() {
        let mut result = MoveToGraveyard::default();
        result.current_step = Step::Reason;
        result
            .handle_input("A reason".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = MoveToGraveyard::default();
        expected.current_step = Step::Done;
        expected.reason = Some("A reason".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_err() {
        let mut action = MoveToGraveyard::default();
        action.current_step = Step::Done;
        let result = action.handle_input("".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn done_done() {
        let mut action = MoveToGraveyard::default();
        action.current_step = Step::Done;
        assert!(action.is_done())
    }

    #[test]
    fn done_notdone() {
        assert!(!MoveToGraveyard::default().is_done())
    }

    #[test]
    fn write_res_no_plant() {
        let result = MoveToGraveyard::default().write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_res_no_died() {
        let mut action = MoveToGraveyard::default();
        action.plant_name = Some("Plant1".to_owned());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_res_no_reason() {
        let mut action = MoveToGraveyard::default();
        action.plant_name = Some("Plant1".to_owned());
        action.died_date = Some(example_date1());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_res() {
        let mut action = MoveToGraveyard::default();
        action.plant_name = Some("Plant1".to_owned());
        action.died_date = Some(example_date1());
        action.reason = Some("a reason".to_owned());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_ok())
    }

    #[test]
    fn next_plant() {
        let result = MoveToGraveyard::default().get_next_prompt().unwrap();
        let expected = "Please enter Plant Name";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_died() {
        let mut action = MoveToGraveyard::default();
        action.current_step = Step::DiedDate;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter Died Date";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_reason() {
        let mut action = MoveToGraveyard::default();
        action.current_step = Step::Reason;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter Died Reason";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_err() {
        let mut action = MoveToGraveyard::default();
        action.current_step = Step::Done;
        let result = action.get_next_prompt();
        assert!(result.is_err())
    }

    #[test]
    fn into_action() {
        let result = <MoveToGraveyard as Into<BotAction>>::into(MoveToGraveyard::default());
        let expected = BotAction::MoveToGraveyard(MoveToGraveyard::default());
        assert_eq!(result, expected)
    }
}
