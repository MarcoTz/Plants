use super::{input_handlers::input_plant_names, Action, BotAction};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::log_item::LogItem;

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
            Ok("Please enter plants to water (separate by comma)".to_owned())
        }
    }
}

impl From<WaterPlants> for BotAction {
    fn from(water: WaterPlants) -> BotAction {
        BotAction::WaterPlants(water)
    }
}
