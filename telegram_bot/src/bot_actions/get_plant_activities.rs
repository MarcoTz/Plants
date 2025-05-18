use super::{input_handlers::input_plant_name, Action, BotAction, DatabaseManager, Error};

#[derive(Debug, PartialEq, Clone)]
pub struct GetPlantActivities {
    plant_name: Option<String>,
    done: bool,
}

impl Default for GetPlantActivities {
    fn default() -> GetPlantActivities {
        GetPlantActivities {
            plant_name: None,
            done: false,
        }
    }
}

impl Action for GetPlantActivities {
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
        let plant = db_man.get_plant(&plant_name)?;
        let mut out_strs = vec![];
        for activity in plant.activities.iter() {
            let note_str = if let Some(ref note) = activity.note {
                note.clone()
            } else {
                "".to_owned()
            };
            let activity_out = format!(
                "\t{} -- {}, note: {}",
                activity.date, activity.activity, note_str
            );
            out_strs.push(activity_out);
        }
        Ok(format!(
            "Activities for {}\n{}",
            plant_name,
            out_strs.join("\n")
        ))
    }
    fn get_next_prompt(&self) -> Result<String, Error> {
        if self.done {
            Err(Error::ActionAlreadyDone("Get Plant Activities".to_owned()))
        } else {
            Ok("Please enter plant to look up".to_owned())
        }
    }
}

impl From<GetPlantActivities> for BotAction {
    fn from(gt: GetPlantActivities) -> BotAction {
        BotAction::GetPlantActivities(gt)
    }
}
