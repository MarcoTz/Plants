use super::{input_handlers::input_plant_names, Action, BotAction};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::log_item::LogItem;

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
            Ok("Plese enter plants to fertilize (separate by comma)".to_owned())
        }
    }
}

impl From<FertilizePlants> for BotAction {
    fn from(fert: FertilizePlants) -> BotAction {
        BotAction::FertilizePlants(fert)
    }
}
