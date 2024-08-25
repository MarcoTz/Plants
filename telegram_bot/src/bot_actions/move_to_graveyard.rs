use super::{input_handlers::input_plant_name, Action, BotAction};
use crate::errors::Error;
use chrono::NaiveDate;
use database::database_manager::DatabaseManager;
use plants::graveyard::GraveyardPlant;

#[derive(PartialEq, Eq)]
enum Step {
    PlantName,
    DiedDate,
    Reason,
    Done,
}
pub struct MoveToGraveyard {
    current_step: Step,
    date_format: String,
    plant_name: Option<String>,
    died_date: Option<NaiveDate>,
    reason: Option<String>,
}

impl MoveToGraveyard {
    pub fn new(date_format: &str) -> MoveToGraveyard {
        MoveToGraveyard {
            current_step: Step::PlantName,
            date_format: date_format.to_owned(),
            plant_name: None,
            died_date: None,
            reason: None,
        }
    }
}

impl Action for MoveToGraveyard {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::PlantName => {
                let name = input_plant_name(input, db_man)?;
                self.plant_name = Some(name);
                self.current_step = Step::DiedDate;
                Ok(())
            }
            Step::DiedDate => {
                let date = NaiveDate::parse_from_str(input.trim(), &self.date_format)
                    .map_err(|_| Error::ParseError("Died Date".to_owned()))?;
                self.died_date = Some(date);
                self.current_step = Step::Reason;
                Ok(())
            }
            Step::Reason => {
                let reason = input.trim().to_owned();
                self.reason = Some(reason);
                self.current_step = Step::Done;
                Ok(())
            }
            Step::Done => Err(Error::ActionAlreadyDone("Move To Graveyard".to_owned())),
        }
    }
    fn is_done(&self) -> bool {
        self.current_step == Step::Done
    }
    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let name = self
            .plant_name
            .clone()
            .ok_or(Error::MissingInput("Plant Name".to_owned()))?;
        let died = self
            .died_date
            .ok_or(Error::MissingInput("Died Date".to_owned()))?;
        let reason = self
            .reason
            .clone()
            .ok_or(Error::MissingInput("Died Reason".to_owned()))?;
        let plant = db_man.get_plant(&name)?;
        let gr_plant = GraveyardPlant {
            name: name.clone(),
            species: plant.species.map(|sp| sp.name).unwrap_or("".to_owned()),
            planted: plant.obtained,
            died,
            reason,
        };
        db_man.kill_plant(gr_plant)?;
        let ret_msg = format!("Successfully moved plant {name} to graveyard");
        Ok(ret_msg)
    }
    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::PlantName => Ok("Please enter Plant Name".to_owned()),
            Step::DiedDate => Ok("Please Enter Died Date".to_owned()),
            Step::Reason => Ok("Please enter Died Reason".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("Move To Graveyard".to_owned())),
        }
    }
}

impl From<MoveToGraveyard> for BotAction {
    fn from(gr: MoveToGraveyard) -> BotAction {
        BotAction::MoveToGraveyard(gr)
    }
}
