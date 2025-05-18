use super::{input_handlers::input_plant_name, Action, BotAction, DatabaseManager, Error};

#[derive(Debug, Clone)]
pub struct GetPlantGrowth {
    plant_name: Option<String>,
    done: bool,
}

impl Default for GetPlantGrowth {
    fn default() -> GetPlantGrowth {
        GetPlantGrowth {
            plant_name: None,
            done: false,
        }
    }
}

impl Action for GetPlantGrowth {
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
            .ok_or(Error::MissingInput("Plant to look up".to_owned()))?;
        let growth = db_man.get_growth_plant(&plant_name)?;
        let mut out_strs = vec![];
        for growth_item in growth {
            let note_str = if let Some(note) = growth_item.note {
                format!("note: {note}")
            } else {
                "".to_owned()
            };
            out_strs.push(format!(
                "\t{}: height: {}, width: {}, health:{},{}",
                growth_item.date,
                growth_item.height_cm,
                growth_item.width_cm,
                growth_item.health,
                note_str
            ));
        }
        Ok(format!(
            "Growth Updates for Plant {}:\n{}",
            plant_name,
            out_strs.join("\n")
        ))
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        if self.done {
            Err(Error::ActionAlreadyDone(
                "Get Growth updates for Plant".to_owned(),
            ))
        } else {
            Ok("Please enter Plant to look up".to_owned())
        }
    }
}

impl From<GetPlantGrowth> for BotAction {
    fn from(gt: GetPlantGrowth) -> BotAction {
        BotAction::GetPlantGrowth(gt)
    }
}
