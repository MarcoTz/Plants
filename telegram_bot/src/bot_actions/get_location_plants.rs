use super::{input_handlers::input_location, Action, BotAction, DatabaseManager, Error};

#[derive(Debug, PartialEq, Clone)]
pub struct GetLocationPlants {
    location: Option<String>,
    done: bool,
}

impl Default for GetLocationPlants {
    fn default() -> GetLocationPlants {
        GetLocationPlants {
            location: None,
            done: false,
        }
    }
}

impl Action for GetLocationPlants {
    fn handle_input<T>(&mut self, input: String, db_man: &mut T) -> Result<(), Error>
    where
        T: DatabaseManager,
    {
        let loc = input_location(input, db_man)?;
        self.location = Some(loc);
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
        let loc_name = self
            .location
            .clone()
            .ok_or(Error::MissingInput("Location to look up".to_owned()))?;
        let plants = db_man.get_plants_by_location(&loc_name)?;
        let mut out_strs = vec![];
        for plant in plants.iter() {
            out_strs.push(format!("\t{}", plant.info.name));
        }
        Ok(format!("Plants at {}:\n{}", loc_name, out_strs.join("\n")))
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        if self.done {
            Err(Error::ActionAlreadyDone(
                "Get Plants at Location".to_owned(),
            ))
        } else {
            Ok("Please enter location to look up".to_owned())
        }
    }
}

impl From<GetLocationPlants> for BotAction {
    fn from(gt: GetLocationPlants) -> BotAction {
        BotAction::GetLocationPlants(gt)
    }
}
