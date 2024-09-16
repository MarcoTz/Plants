use super::{
    input_handlers::{input_health, input_plant_name},
    Action, BotAction,
};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::growth_item::GrowthItem;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Step {
    PlantName,
    Height,
    Width,
    Health,
    Note,
    Done,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NewGrowth {
    current_step: Step,
    name: Option<String>,
    height: Option<f32>,
    width: Option<f32>,
    health: Option<i32>,
    note: Option<String>,
}

impl NewGrowth {
    pub fn new() -> NewGrowth {
        NewGrowth {
            current_step: Step::PlantName,
            name: None,
            height: None,
            width: None,
            health: None,
            note: None,
        }
    }
}

impl Default for NewGrowth {
    fn default() -> Self {
        NewGrowth::new()
    }
}

impl Action for NewGrowth {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::PlantName => {
                let name = input_plant_name(input, db_man)?;
                self.name = Some(name);
                self.current_step = Step::Height;
                Ok(())
            }
            Step::Height => {
                let height = input
                    .to_lowercase()
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Height".to_owned()))?;
                self.height = Some(height);
                self.current_step = Step::Width;
                Ok(())
            }
            Step::Width => {
                let width = input
                    .to_lowercase()
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Width".to_owned()))?;
                self.width = Some(width);
                self.current_step = Step::Health;
                Ok(())
            }
            Step::Health => {
                let health = input_health(input)?;
                self.health = Some(health);
                self.current_step = Step::Note;
                Ok(())
            }
            Step::Note => {
                if input.to_lowercase().trim() == "done" {
                    self.note = None
                } else {
                    self.note = Some(input)
                }
                self.current_step = Step::Done;
                Ok(())
            }
            Step::Done => Err(Error::ActionAlreadyDone("Add Growth".to_owned())),
        }
    }
    fn is_done(&self) -> bool {
        self.current_step == Step::Done
    }

    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let plant = self
            .name
            .clone()
            .ok_or(Error::MissingInput("Plant Name".to_owned()))?;
        let height_cm = self
            .height
            .ok_or(Error::MissingInput("Plant Height".to_owned()))?;
        let width_cm = self
            .width
            .ok_or(Error::MissingInput("Plant Width".to_owned()))?;
        let health = self
            .health
            .ok_or(Error::MissingInput("Plant Health".to_owned()))?;
        let growth = GrowthItem {
            plant: plant.clone(),
            height_cm,
            width_cm,
            health,
            note: self.note.clone(),
            date: Local::now().date_naive(),
        };
        db_man.write_growth(growth)?;
        let ret_msg = format!(
            "Successfully added growth {height_cm}x{width_cm}, {health}, {} for {plant}",
            self.note.clone().unwrap_or("".to_owned())
        );
        Ok(ret_msg)
    }
    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::PlantName => Ok("Please enter plant name".to_owned()),
            Step::Height => Ok("Please enter height (cm)".to_owned()),
            Step::Width => Ok("Please enter width (cm)".to_owned()),
            Step::Health => Ok("Please enter health (0-5)".to_owned()),
            Step::Note => Ok("Please enter note (enter \"Done\" for no note)".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("New Growth".to_owned())),
        }
    }
}

impl From<NewGrowth> for BotAction {
    fn from(growth: NewGrowth) -> BotAction {
        BotAction::NewGrowth(growth)
    }
}

#[cfg(test)]
mod new_growth_tests {
    use super::{Action, NewGrowth, Step};
    use crate::test_common::DummyManager;

    #[test]
    fn growth_default() {
        let result = NewGrowth::default();
        let expected = NewGrowth {
            current_step: Step::PlantName,
            name: None,
            height: None,
            width: None,
            health: None,
            note: None,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plant() {
        let mut result = NewGrowth::default();
        result
            .handle_input("Plant1".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewGrowth::default();
        expected.current_step = Step::Height;
        expected.name = Some("Plant1".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plant_err() {
        let mut action = NewGrowth::default();
        let result = action.handle_input("not a plant".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_height() {
        let mut result = NewGrowth::default();
        result.current_step = Step::Height;
        result
            .handle_input("1.0".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewGrowth::default();
        expected.current_step = Step::Width;
        expected.height = Some(1.0);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_height_err() {
        let mut action = NewGrowth::default();
        action.current_step = Step::Height;
        let result = action.handle_input("not a number".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_width() {
        let mut result = NewGrowth::default();
        result.current_step = Step::Width;
        result
            .handle_input("1.0".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewGrowth::default();
        expected.current_step = Step::Health;
        expected.width = Some(1.0);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_widht_err() {
        let mut action = NewGrowth::default();
        action.current_step = Step::Width;
        let result = action.handle_input("not a number".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_health() {
        let mut result = NewGrowth::default();
        result.current_step = Step::Health;
        result
            .handle_input("3".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewGrowth::default();
        expected.current_step = Step::Note;
        expected.health = Some(3);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_health_err() {
        let mut action = NewGrowth::default();
        action.current_step = Step::Health;
        let result = action.handle_input("6".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_note_some() {
        let mut result = NewGrowth::default();
        result.current_step = Step::Note;
        result
            .handle_input("note".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewGrowth::default();
        expected.current_step = Step::Done;
        expected.note = Some("note".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_note_none() {
        let mut result = NewGrowth::default();
        result.current_step = Step::Note;
        result
            .handle_input("Done".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewGrowth::default();
        expected.current_step = Step::Done;
        expected.note = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn done_done() {
        let mut action = NewGrowth::default();
        action.current_step = Step::Done;
        assert!(action.is_done())
    }

    #[test]
    fn done_notdone() {
        assert!(!NewGrowth::default().is_done())
    }

    #[test]
    fn write_no_plant() {
        let result = NewGrowth::default().write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_height() {
        let mut action = NewGrowth::default();
        action.name = Some("Plant1".to_owned());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err());
    }

    #[test]
    fn write_no_width() {
        let mut action = NewGrowth::default();
        action.name = Some("Plant1".to_owned());
        action.height = Some(1.0);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_health() {
        let mut action = NewGrowth::default();
        action.name = Some("Plant1".to_owned());
        action.height = Some(1.0);
        action.width = Some(1.0);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write() {
        let mut action = NewGrowth::default();
        action.name = Some("Plant1".to_owned());
        action.height = Some(1.0);
        action.width = Some(1.0);
        action.health = Some(3);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_ok())
    }

    #[test]
    fn next_plant() {
        let result = NewGrowth::default().get_next_prompt().unwrap();
        let expected = "Please enter plant name";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_height() {
        let mut action = NewGrowth::default();
        action.current_step = Step::Height;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter height (cm)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_width() {
        let mut action = NewGrowth::default();
        action.current_step = Step::Width;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter width (cm)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_health() {
        let mut action = NewGrowth::default();
        action.current_step = Step::Health;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter health (0-5)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_note() {
        let mut action = NewGrowth::default();
        action.current_step = Step::Note;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter note (enter \"Done\" for no note)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_err() {
        let mut action = NewGrowth::default();
        action.current_step = Step::Done;
        let result = action.get_next_prompt();
        assert!(result.is_err())
    }
}
