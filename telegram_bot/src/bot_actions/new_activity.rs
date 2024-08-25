use super::{input_handlers::input_plant_names, Action, BotAction};
use crate::errors::Error;
use chrono::NaiveDate;
use database::database_manager::DatabaseManager;
use plants::log_item::LogItem;

#[derive(PartialEq, Eq)]
enum Step {
    Date,
    Activity,
    Plants,
    Note,
    Done,
}

pub struct NewActivity {
    current_step: Step,
    date_format: String,
    date: Option<NaiveDate>,
    activity: Option<String>,
    plants: Option<Vec<String>>,
    note: Option<String>,
}

impl NewActivity {
    pub fn new(date_format: &str) -> NewActivity {
        NewActivity {
            current_step: Step::Date,
            date_format: date_format.to_owned(),
            date: None,
            activity: None,
            plants: None,
            note: None,
        }
    }
}

impl Action for NewActivity {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::Date => {
                let date =
                    NaiveDate::parse_from_str(input.to_lowercase().trim(), &self.date_format)
                        .map_err(|_| Error::ParseError("Activity Date".to_owned()))?;
                self.date = Some(date);
                self.current_step = Step::Activity;
                Ok(())
            }
            Step::Activity => {
                self.activity = Some(input.trim().to_owned());
                self.current_step = Step::Plants;
                Ok(())
            }
            Step::Plants => {
                let plants = input_plant_names(input, db_man)?;
                self.plants = Some(plants);
                self.current_step = Step::Note;
                Ok(())
            }
            Step::Note => {
                if input.to_lowercase().trim() == "done" {
                    self.note = None;
                } else {
                    self.note = Some(input);
                }
                self.current_step = Step::Done;
                Ok(())
            }
            Step::Done => Err(Error::ActionAlreadyDone("Adding Activity".to_owned())),
        }
    }
    fn is_done(&self) -> bool {
        self.current_step == Step::Done
    }
    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let activity = self
            .activity
            .clone()
            .ok_or(Error::MissingInput("Activty Name".to_owned()))?;
        let date = self
            .date
            .ok_or(Error::MissingInput("Activity Date".to_owned()))?;
        let plants = self
            .plants
            .clone()
            .ok_or(Error::MissingInput("Plant Name".to_owned()))?;
        let mut log_items = vec![];
        for plant in plants.iter().cloned() {
            let log = LogItem {
                activity: activity.clone(),
                date,
                plant,
                note: self.note.clone(),
            };
            log_items.push(log)
        }
        db_man.write_logs(log_items)?;
        let ret_msg = format!(
            "Successfully wrote log {activity} for plants {} ({})",
            plants.join(", "),
            date.format(&self.date_format)
        );
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::Date => Ok(format!("Enter Activity Date ({})", self.date_format)),
            Step::Activity => Ok("Please enter activity".to_owned()),
            Step::Plants => Ok("Please enter affected plants (separated by comma)".to_owned()),
            Step::Note => Ok("Please enter Note (enter \"Done\" for no note".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("Activity".to_owned())),
        }
    }
}

impl From<NewActivity> for BotAction {
    fn from(act: NewActivity) -> BotAction {
        BotAction::NewActivity(act)
    }
}
