use super::{Action, BotAction};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::log_item::LogItem;

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
        let location_plants = db_man.get_plants_by_location(input.trim().to_owned())?;
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
