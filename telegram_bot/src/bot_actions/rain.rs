use super::{Action, BotAction};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::{location::Location, log_item::LogItem, named::Named};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Rain;

impl Action for Rain {
    fn handle_input<T: DatabaseManager>(&mut self, _: String, _: &mut T) -> Result<(), Error> {
        Err(Error::ActionAlreadyDone("Rain".to_owned()))
    }
    fn is_done(&self) -> bool {
        true
    }
    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let locations = db_man.get_locations()?;
        let outside_locations: Vec<Location> =
            locations.into_iter().filter(|loc| loc.outside).collect();
        let mut outside_plants = vec![];
        for outside_location in outside_locations.iter() {
            let location_plants = db_man.get_plants_by_location(&outside_location.get_name())?;
            outside_plants.extend(location_plants);
        }

        let mut watering_items = vec![];
        for plant in outside_plants.iter() {
            let plant_log = LogItem {
                activity: "Watering".to_owned(),
                date: Local::now().date_naive(),
                plant: plant.get_name(),
                note: None,
            };
            watering_items.push(plant_log);
        }

        db_man.write_logs(watering_items)?;
        let ret_msg = format!(
            "Successfully watered plants: {}",
            outside_plants
                .iter()
                .map(|pl| pl.get_name())
                .collect::<Vec<String>>()
                .join(", ")
        );
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        Err(Error::ActionAlreadyDone("Rain".to_owned()))
    }
}

impl From<Rain> for BotAction {
    fn from(rain: Rain) -> BotAction {
        BotAction::Rain(rain)
    }
}

#[cfg(test)]
mod rain_tests {
    use super::{Action, BotAction, Rain};
    use crate::test_common::DummyManager;

    #[test]
    fn rain_default() {
        let result = Rain::default();
        let expected = Rain {};
        assert_eq!(result, expected)
    }

    #[test]
    fn input_err() {
        let result = Rain::default().handle_input("".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn done_done() {
        assert!(Rain::default().is_done())
    }

    #[test]
    fn write_res() {
        let result = Rain::default().write_result(&mut DummyManager {});
        assert!(result.is_ok())
    }

    #[test]
    fn next_err() {
        let result = Rain::default().get_next_prompt();
        assert!(result.is_err())
    }

    #[test]
    fn into_action() {
        let result = <Rain as Into<BotAction>>::into(Rain::default());
        let expected = BotAction::Rain(Rain::default());
        assert_eq!(result, expected)
    }
}
