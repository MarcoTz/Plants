use super::{Action, BotAction};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::log_item::LogItem;

pub struct FertilizePlants {
    fertilized_plants: Vec<String>,
    done: bool,
}

impl Action for FertilizePlants {
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
            self.fertilized_plants.push(plant.to_owned());
        }
        self.done = true;
        Ok(())
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let mut activities = vec![];
        for plant in self.fertilized_plants.iter().cloned() {
            activities.push(LogItem {
                activity: "Fertilizing".to_owned(),
                date: Local::now().date_naive(),
                plant,
                note: None,
            });
        }
        db_man.write_logs(activities)?;
        let ret_msg = format!(
            "Successfully fertilized plants {}",
            self.fertilized_plants.join(", ")
        );
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        if self.done {
            Err(Error::ActionAlreadyDone("Fertilize Plants".to_owned()))
        } else {
            Ok("Plese enter plants to fertilize (separate by comma)".to_owned())
        }
    }
}

impl From<FertilizePlants> for BotAction {
    fn from(fert: FertilizePlants) -> BotAction {
        BotAction::FertilizePlants(fert)
    }
}
