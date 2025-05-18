use super::{input_handlers::input_plant_name, Action, BotAction, DatabaseManager, Error};

#[derive(Debug, PartialEq, Clone)]
pub struct GetPlantDetails {
    plant_name: Option<String>,
    done: bool,
}

impl Default for GetPlantDetails {
    fn default() -> GetPlantDetails {
        GetPlantDetails {
            plant_name: None,
            done: false,
        }
    }
}

impl Action for GetPlantDetails {
    fn handle_input<T>(&mut self, input: String, db_man: &mut T) -> Result<(), Error>
    where
        T: DatabaseManager,
    {
        let plant_name = input_plant_name(input, db_man)?;
        self.plant_name = Some(plant_name);
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
            .ok_or(Error::MissingInput("Location to look up".to_owned()))?;
        let plant = db_man.get_plant(&plant_name)?;
        Ok(format!("{} ({})\n\tLocation:{}\n\tOrigin:{}\n\tObtained:{}\n\tAutomatically Watered:{}\n\tNotes:{}",
        plant.info.name,
        plant.info.species,
        plant.info.location,
        plant.info.origin,
        plant.info.obtained,
        plant.info.auto_water,
        plant.info.notes.join(", ")))
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        if self.done {
            Err(Error::ActionAlreadyDone("Get Plant Details".to_owned()))
        } else {
            Ok("Please enter plant to look up".to_owned())
        }
    }
}

impl From<GetPlantDetails> for BotAction {
    fn from(gt: GetPlantDetails) -> BotAction {
        BotAction::GetPlantDetails(gt)
    }
}
