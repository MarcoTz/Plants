use super::{
    input_handlers::{input_health, input_plant_name},
    Action, BotAction,
};
use crate::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use plants::growth_item::GrowthItem;

#[derive(Clone, PartialEq, Eq)]
enum Step {
    PlantName,
    Height,
    Width,
    Health,
    Note,
    Done,
}

#[derive(Clone)]
pub struct NewGrowth {
    current_step: Step,
    name: Option<String>,
    height: Option<f32>,
    width: Option<f32>,
    health: Option<i32>,
    note: Option<String>,
}

impl NewGrowth {
    pub fn new() -> NewGrowth {
        NewGrowth {
            current_step: Step::PlantName,
            name: None,
            height: None,
            width: None,
            health: None,
            note: None,
        }
    }
}

impl Default for NewGrowth {
    fn default() -> Self {
        NewGrowth::new()
    }
}

impl Action for NewGrowth {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::PlantName => {
                let name = input_plant_name(input, db_man)?;
                self.name = Some(name);
                self.current_step = Step::Height;
                Ok(())
            }
            Step::Height => {
                let height = input
                    .to_lowercase()
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Height".to_owned()))?;
                self.height = Some(height);
                self.current_step = Step::Width;
                Ok(())
            }
            Step::Width => {
                let width = input
                    .to_lowercase()
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Width".to_owned()))?;
                self.width = Some(width);
                self.current_step = Step::Health;
                Ok(())
            }
            Step::Health => {
                let health = input_health(input)?;
                self.health = Some(health);
                self.current_step = Step::Note;
                Ok(())
            }
            Step::Note => {
                if input.to_lowercase().trim() == "done" {
                    self.note = None
                } else {
                    self.note = Some(input)
                }
                self.current_step = Step::Done;
                Ok(())
            }
            Step::Done => Err(Error::ActionAlreadyDone("Add Growth".to_owned())),
        }
    }
    fn is_done(&self) -> bool {
        self.current_step == Step::Done
    }

    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let plant = self
            .name
            .clone()
            .ok_or(Error::MissingInput("Plant Name".to_owned()))?;
        let height_cm = self
            .height
            .ok_or(Error::MissingInput("Plant Height".to_owned()))?;
        let width_cm = self
            .width
            .ok_or(Error::MissingInput("Plant Width".to_owned()))?;
        let health = self
            .health
            .ok_or(Error::MissingInput("Plant Health".to_owned()))?;
        let growth = GrowthItem {
            plant: plant.clone(),
            height_cm,
            width_cm,
            health,
            note: self.note.clone(),
            date: Local::now().date_naive(),
        };
        db_man.write_growth(growth)?;
        let ret_msg = format!(
            "Successfully added growth {height_cm}x{width_cm}, {health}, {} for {plant}",
            self.note.clone().unwrap_or("".to_owned())
        );
        Ok(ret_msg)
    }
    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::PlantName => Ok("Please enter plant name".to_owned()),
            Step::Height => Ok("Please enter height (cm)".to_owned()),
            Step::Width => Ok("Please enter width (cm)".to_owned()),
            Step::Health => Ok("Please enter health (0-5)".to_owned()),
            Step::Note => Ok("Please enter note (enter \"Done\" for no note)".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("New Growth".to_owned())),
        }
    }
}

impl From<NewGrowth> for BotAction {
    fn from(growth: NewGrowth) -> BotAction {
        BotAction::NewGrowth(growth)
    }
}
