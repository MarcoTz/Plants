use super::{Action, BotAction};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::log_item::LogItem;

pub struct WaterPlants {
    watered_plants: Vec<String>,
    done: bool,
}

impl Action for WaterPlants {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        let plants = input.split(",").map(|st| st.trim());
        for plant in plants {
            let exists = db_man.plant_exists(plant.to_owned())?;
            let _ = if exists {
                Ok(())
            } else {
                Err(Error::PlantDoesNotExist(plant.to_owned()))
            }?;
            self.watered_plants.push(plant.to_owned());
        }
        self.done = true;
        Ok(())
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let mut activities = vec![];
        for plant in self.watered_plants.clone() {
            activities.push(LogItem {
                activity: "Watering".to_owned(),
                date: Local::now().date_naive(),
                plant,
                note: None,
            });
        }

        db_man.write_logs(activities)?;
        let ret_msg = format!(
            "Successfully watered plants {}",
            self.watered_plants.clone().join(",")
        );
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
