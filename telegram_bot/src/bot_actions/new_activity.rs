use super::{input_handlers::input_plant_names, Action, BotAction};
use crate::errors::Error;
use chrono::NaiveDate;
use database::database_manager::DatabaseManager;
use plants::log_item::LogItem;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Step {
    Date,
    Activity,
    Plants,
    Note,
    Done,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewActivity {
    current_step: Step,
    date_format: String,
    date: Option<NaiveDate>,
    activity: Option<String>,
    plants: Option<Vec<String>>,
    note: Option<String>,
}

impl NewActivity {
    pub fn new(date_format: &str) -> NewActivity {
        NewActivity {
            current_step: Step::Date,
            date_format: date_format.to_owned(),
            date: None,
            activity: None,
            plants: None,
            note: None,
        }
    }
}

impl Default for NewActivity {
    fn default() -> Self {
        NewActivity::new("%d.%m.%Y")
    }
}

impl Action for NewActivity {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::Date => {
                let date =
                    NaiveDate::parse_from_str(input.to_lowercase().trim(), &self.date_format)
                        .map_err(|_| Error::ParseError("Activity Date".to_owned()))?;
                self.date = Some(date);
                self.current_step = Step::Activity;
                Ok(())
            }
            Step::Activity => {
                self.activity = Some(input.trim().to_owned());
                self.current_step = Step::Plants;
                Ok(())
            }
            Step::Plants => {
                let plants = input_plant_names(input, db_man)?;
                self.plants = Some(plants);
                self.current_step = Step::Note;
                Ok(())
            }
            Step::Note => {
                if input.to_lowercase().trim() == "done" {
                    self.note = None;
                } else {
                    self.note = Some(input);
                }
                self.current_step = Step::Done;
                Ok(())
            }
            Step::Done => Err(Error::ActionAlreadyDone("Adding Activity".to_owned())),
        }
    }
    fn is_done(&self) -> bool {
        self.current_step == Step::Done
    }
    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let activity = self
            .activity
            .clone()
            .ok_or(Error::MissingInput("Activty Name".to_owned()))?;
        let date = self
            .date
            .ok_or(Error::MissingInput("Activity Date".to_owned()))?;
        let plants = self
            .plants
            .clone()
            .ok_or(Error::MissingInput("Plant Name".to_owned()))?;
        let mut log_items = vec![];
        for plant in plants.iter().cloned() {
            let log = LogItem {
                activity: activity.clone(),
                date,
                plant,
                note: self.note.clone(),
            };
            log_items.push(log)
        }
        db_man.write_logs(log_items)?;
        let ret_msg = format!(
            "Successfully wrote log {activity} for plants {} ({})",
            plants.join(", "),
            date.format(&self.date_format)
        );
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::Date => Ok(format!("Enter Activity Date ({})", self.date_format)),
            Step::Activity => Ok("Please enter activity".to_owned()),
            Step::Plants => Ok("Please enter affected plants (separated by comma)".to_owned()),
            Step::Note => Ok("Please enter Note (enter \"Done\" for no note)".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("Activity".to_owned())),
        }
    }
}

impl From<NewActivity> for BotAction {
    fn from(act: NewActivity) -> BotAction {
        BotAction::NewActivity(act)
    }
}

#[cfg(test)]
mod new_activity_tests {
    use super::{Action, BotAction, NewActivity, Step};
    use crate::test_common::{example_date1, DummyManager};

    #[test]
    fn ativity_default() {
        let result = NewActivity::default();
        let expected = NewActivity {
            current_step: Step::Date,
            date_format: "%d.%m.%Y".to_owned(),
            date: None,
            activity: None,
            plants: None,
            note: None,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn input_date() {
        let mut result = NewActivity::default();
        result
            .handle_input("01.01.1970".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewActivity::default();
        expected.current_step = Step::Activity;
        expected.date = Some(example_date1());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_date_err() {
        let result =
            NewActivity::default().handle_input("not a date".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_activity() {
        let mut action = NewActivity::default();
        action.current_step = Step::Activity;
        action
            .handle_input("activity".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewActivity::default();
        expected.current_step = Step::Plants;
        expected.activity = Some("activity".to_owned());
        assert_eq!(action, expected)
    }

    #[test]
    fn input_plants() {
        let mut action = NewActivity::default();
        action.current_step = Step::Plants;
        action
            .handle_input("Plant1,Plant2".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewActivity::default();
        expected.current_step = Step::Note;
        expected.plants = Some(vec!["Plant1".to_owned(), "Plant2".to_owned()]);
        assert_eq!(action, expected)
    }

    #[test]
    fn input_plants_err() {
        let mut action = NewActivity::default();
        action.current_step = Step::Plants;
        let result = action.handle_input("not a plant".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_note_some() {
        let mut action = NewActivity::default();
        action.current_step = Step::Note;
        action
            .handle_input("note".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewActivity::default();
        expected.current_step = Step::Done;
        expected.note = Some("note".to_owned());
        assert_eq!(action, expected)
    }

    #[test]
    fn input_note_none() {
        let mut action = NewActivity::default();
        action.current_step = Step::Note;
        action
            .handle_input("Done".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewActivity::default();
        expected.current_step = Step::Done;
        expected.note = None;
        assert_eq!(action, expected)
    }

    #[test]
    fn input_err() {
        let mut action = NewActivity::default();
        action.current_step = Step::Done;
        let result = action.handle_input("".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn done_done() {
        let mut action = NewActivity::default();
        action.current_step = Step::Done;
        assert!(action.is_done())
    }

    #[test]
    fn done_notdone() {
        assert!(!NewActivity::default().is_done())
    }

    #[test]
    fn write_no_date() {
        let result = NewActivity::default().write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_activity() {
        let mut action = NewActivity::default();
        action.date = Some(example_date1());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_plants() {
        let mut action = NewActivity::default();
        action.date = Some(example_date1());
        action.activity = Some("activity".to_owned());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write() {
        let mut action = NewActivity::default();
        action.date = Some(example_date1());
        action.activity = Some("activity".to_owned());
        action.plants = Some(vec!["Plant1".to_owned()]);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_ok())
    }

    #[test]
    fn next_date() {
        let result = NewActivity::default().get_next_prompt().unwrap();
        let expected = "Enter Activity Date (%d.%m.%Y)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_activity() {
        let mut action = NewActivity::default();
        action.current_step = Step::Activity;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter activity";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_plants() {
        let mut action = NewActivity::default();
        action.current_step = Step::Plants;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter affected plants (separated by comma)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_note() {
        let mut action = NewActivity::default();
        action.current_step = Step::Note;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter Note (enter \"Done\" for no note)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_err() {
        let mut action = NewActivity::default();
        action.current_step = Step::Done;
        let result = action.get_next_prompt();
        assert!(result.is_err())
    }

    #[test]
    fn into_botaction() {
        let result = <NewActivity as Into<BotAction>>::into(NewActivity::default());
        let expected = BotAction::NewActivity(NewActivity::default());
        assert_eq!(result, expected)
    }
}
