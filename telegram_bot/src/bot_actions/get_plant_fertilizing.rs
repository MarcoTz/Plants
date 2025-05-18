use super::{input_handlers::input_plant_name, Action, BotAction, DatabaseManager, Error};
use plants::log_item::LogItem;

#[derive(Debug, PartialEq, Clone)]
pub struct GetPlantFertilizing {
    plant_name: Option<String>,
    done: bool,
}

impl Default for GetPlantFertilizing {
    fn default() -> GetPlantFertilizing {
        GetPlantFertilizing {
            plant_name: None,
            done: false,
        }
    }
}

impl Action for GetPlantFertilizing {
    fn handle_input<T>(&mut self, input: String, db_man: &mut T) -> Result<(), Error>
    where
        T: DatabaseManager,
    {
        let name = input_plant_name(input, db_man)?;
        self.plant_name = Some(name);
        self.done = true;
        Ok(())
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn write_result<T>(&self, db_man: &mut T) -> Result<String, Error>
    where
        T: DatabaseManager,
    {
        let plant_name = self
            .plant_name
            .clone()
            .ok_or(Error::MissingInput("Plant to look up".to_owned()))?;
        let fertilizing: Vec<LogItem> = db_man.get_logs_plant(&plant_name)?;
        let mut out_strs = vec![];
        for fertilizing in fertilizing.iter() {
            out_strs.push(format!("\t{}", fertilizing.date));
        }
        Ok(format!(
            "Fertilizing dates for {}:\n{}",
            plant_name,
            out_strs.join("\n")
        ))
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        if self.done {
            Err(Error::ActionAlreadyDone("Get Plant Fertilizing".to_owned()))
        } else {
            Ok("Please enter plant to look up".to_owned())
        }
    }
}

impl From<GetPlantFertilizing> for BotAction {
    fn from(gt: GetPlantFertilizing) -> BotAction {
        BotAction::GetPlantFertilizing(gt)
    }
}
