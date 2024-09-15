use super::{input_handlers::input_plant_names, Action, BotAction};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::log_item::LogItem;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FertilizePlants {
    fertilized_plants: Option<Vec<String>>,
    done: bool,
}

impl FertilizePlants {
    pub fn new() -> FertilizePlants {
        FertilizePlants {
            fertilized_plants: None,
            done: false,
        }
    }
}

impl Default for FertilizePlants {
    fn default() -> Self {
        FertilizePlants::new()
    }
}

impl Action for FertilizePlants {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        let plants = input_plant_names(input, db_man)?;
        self.fertilized_plants = Some(plants);
        self.done = true;
        Ok(())
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let mut activities = vec![];
        let plants = self
            .fertilized_plants
            .clone()
            .ok_or(Error::MissingInput("Plants to fertilize".to_owned()))?;
        for plant in plants.iter().cloned() {
            activities.push(LogItem {
                activity: "Fertilizing".to_owned(),
                date: Local::now().date_naive(),
                plant,
                note: None,
            });
        }
        db_man.write_logs(activities)?;
        let ret_msg = format!("Successfully fertilized plants {}", plants.join(", "));
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        if self.done {
            Err(Error::ActionAlreadyDone("Fertilize Plants".to_owned()))
        } else {
            Ok("Please enter plants to fertilize (separate by comma)".to_owned())
        }
    }
}

impl From<FertilizePlants> for BotAction {
    fn from(fert: FertilizePlants) -> BotAction {
        BotAction::FertilizePlants(fert)
    }
}

#[cfg(test)]
mod fertilize_plants_tests {
    use super::{Action, FertilizePlants};
    use crate::test_common::DummyManager;

    #[test]
    fn fertilize_default() {
        let result = FertilizePlants::default();
        let expected = FertilizePlants {
            fertilized_plants: None,
            done: false,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn input() {
        let mut action = FertilizePlants::default();
        action
            .handle_input("Plant1,Plant2".to_owned(), &mut DummyManager {})
            .unwrap();
        let expected = FertilizePlants {
            fertilized_plants: Some(vec!["Plant1".to_owned(), "Plant2".to_owned()]),
            done: true,
        };
        assert_eq!(action, expected)
    }

    #[test]
    fn done() {
        let mut result = FertilizePlants::default();
        result.done = true;
        assert!(result.is_done())
    }

    #[test]
    fn not_done() {
        let result = FertilizePlants::default();
        assert!(!result.is_done())
    }

    #[test]
    fn write() {
        let mut action = FertilizePlants::default();
        action.fertilized_plants = Some(vec![]);
        let result = action.write_result(&mut DummyManager {}).unwrap();
        let expected = "Successfully fertilized plants ";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_prompt() {
        let result = FertilizePlants::default().get_next_prompt().unwrap();
        let expected = "Please enter plants to fertilize (separate by comma)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_prompt_fail() {
        let mut action = FertilizePlants::default();
        action.done = true;
        let result = action.get_next_prompt();
        assert!(result.is_err())
    }
}
