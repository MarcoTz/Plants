use super::{input_handlers::input_plant_names, Action, BotAction};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::log_item::LogItem;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WaterPlants {
    watered_plants: Option<Vec<String>>,
    done: bool,
}

impl WaterPlants {
    pub fn new() -> WaterPlants {
        WaterPlants {
            watered_plants: None,
            done: false,
        }
    }
}

impl Default for WaterPlants {
    fn default() -> Self {
        WaterPlants::new()
    }
}

impl Action for WaterPlants {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        let plants = input_plant_names(input, db_man)?;
        self.watered_plants = Some(plants);
        self.done = true;
        Ok(())
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let mut activities = vec![];
        let plants = self
            .watered_plants
            .clone()
            .ok_or(Error::MissingInput("Plants to water".to_owned()))?;
        for plant in plants.clone() {
            activities.push(LogItem {
                activity: "Watering".to_owned(),
                date: Local::now().date_naive(),
                plant,
                note: None,
            });
        }

        db_man.write_logs(activities)?;
        let ret_msg = format!("Successfully watered plants {}", plants.clone().join(","));
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        if self.done {
            Err(Error::ActionAlreadyDone("Water plants".to_owned()))
        } else {
            Ok("Please enter watered plants (separate by comma)".to_owned())
        }
    }
}

impl From<WaterPlants> for BotAction {
    fn from(water: WaterPlants) -> BotAction {
        BotAction::WaterPlants(water)
    }
}

#[cfg(test)]
mod water_plants_tests {
    use super::{Action, BotAction, WaterPlants};
    use crate::test_common::DummyManager;

    #[test]
    fn water_default() {
        let result = WaterPlants::default();
        let expected = WaterPlants {
            watered_plants: None,
            done: false,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plants() {
        let mut result = WaterPlants::default();
        result
            .handle_input("Plant1,Plant2".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = WaterPlants::default();
        expected.watered_plants = Some(vec!["Plant1".to_owned(), "Plant2".to_owned()]);
        expected.done = true;
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plants_err() {
        let result =
            WaterPlants::default().handle_input("Not a plant".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn done_done() {
        let mut action = WaterPlants::default();
        action.done = true;
        assert!(action.is_done())
    }

    #[test]
    fn done_notdone() {
        assert!(!WaterPlants::default().is_done())
    }

    #[test]
    fn write_no_plants() {
        let result = WaterPlants::default().write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write() {
        let mut action = WaterPlants::default();
        action.watered_plants = Some(vec!["Plant1".to_owned()]);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_ok())
    }

    #[test]
    fn next_plants() {
        let result = WaterPlants::default().get_next_prompt().unwrap();
        let expected = "Please enter watered plants (separate by comma)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_err() {
        let mut action = WaterPlants::default();
        action.done = true;
        let result = action.get_next_prompt();
        assert!(result.is_err())
    }

    #[test]
    fn into_action() {
        let result = <WaterPlants as Into<BotAction>>::into(WaterPlants::default());
        let expected = BotAction::WaterPlants(WaterPlants::default());
        assert_eq!(result, expected)
    }
}
