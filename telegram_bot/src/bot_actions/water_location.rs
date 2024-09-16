use super::{Action, BotAction};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::log_item::LogItem;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WaterLocation {
    watered_plants: Option<Vec<String>>,
    done: bool,
}

impl WaterLocation {
    pub fn new() -> WaterLocation {
        WaterLocation {
            watered_plants: None,
            done: false,
        }
    }
}

impl Default for WaterLocation {
    fn default() -> Self {
        WaterLocation::new()
    }
}

impl Action for WaterLocation {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        let location_plants = db_man.get_plants_by_location(input.trim())?;
        if location_plants.is_empty() {
            Err(Error::NoPlantsLocation(input))
        } else {
            self.watered_plants = Some(
                location_plants
                    .iter()
                    .map(|pl| pl.info.name.clone())
                    .collect(),
            );
            self.done = true;
            Ok(())
        }
    }
    fn is_done(&self) -> bool {
        self.done
    }

    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let mut activities = vec![];
        let plants = self
            .watered_plants
            .clone()
            .ok_or(Error::MissingInput("Location to water".to_owned()))?;
        for plant in plants.iter().cloned() {
            activities.push(LogItem {
                activity: "Watering".to_owned(),
                date: Local::now().date_naive(),
                plant,
                note: None,
            });
        }
        db_man.write_logs(activities)?;
        let ret_msg = format!("Successfully watered plants {}", plants.join(", "));
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        if self.done {
            Err(Error::ActionAlreadyDone("Watering Location".to_owned()))
        } else {
            Ok("Please enter location to water".to_owned())
        }
    }
}

impl From<WaterLocation> for BotAction {
    fn from(water: WaterLocation) -> BotAction {
        BotAction::WaterLocation(water)
    }
}

#[cfg(test)]
mod water_location_tests {
    use super::{Action, BotAction, WaterLocation};
    use crate::test_common::DummyManager;

    #[test]
    fn water_default() {
        let result = WaterLocation::default();
        let expected = WaterLocation {
            watered_plants: None,
            done: false,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn input_loc() {
        let mut result = WaterLocation::default();
        result
            .handle_input("Inside".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = WaterLocation::default();
        expected.watered_plants = Some(vec!["A Plant".to_owned()]);
        expected.done = true;
        assert_eq!(result, expected)
    }

    #[test]
    fn input_loc_err() {
        let result = WaterLocation::default()
            .handle_input("not a location".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn done_done() {
        let mut action = WaterLocation::default();
        action.done = true;
        assert!(action.is_done())
    }

    #[test]
    fn done_notdone() {
        assert!(!WaterLocation::default().is_done())
    }

    #[test]
    fn write_no_loc() {
        let result = WaterLocation::default().write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write() {
        let mut action = WaterLocation::default();
        action.watered_plants = Some(vec!["Plant1".to_owned()]);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_ok())
    }

    #[test]
    fn next_loc() {
        let result = WaterLocation::default().get_next_prompt().unwrap();
        let expected = "Please enter location to water";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_err() {
        let mut action = WaterLocation::default();
        action.done = true;
        let result = action.get_next_prompt();
        assert!(result.is_err())
    }

    #[test]
    fn into_action() {
        let result = <WaterLocation as Into<BotAction>>::into(WaterLocation::default());
        let expected = BotAction::WaterLocation(WaterLocation::default());
        assert_eq!(result, expected)
    }
}
