use super::{
    input_handlers::{input_health, input_species},
    Action, BotAction,
};
use crate::errors::Error;
use chrono::Local;
use chrono::NaiveDate;
use database::database_manager::DatabaseManager;
use plants::{
    growth_item::GrowthItem,
    plant::{PlantInfo, PlantLocation, PlantSpecies},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Step {
    PlantName,
    SpeciesName,
    Height,
    Width,
    Health,
    Location,
    AutoWatered,
    Origin,
    ObtainedDate,
    Notes,
    Done,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NewPlant {
    current_step: Step,
    date_format: String,
    plant_name: Option<String>,
    species_name: Option<String>,
    height: Option<f32>,
    width: Option<f32>,
    health: Option<i32>,
    location: Option<PlantLocation>,
    autowatered: Option<bool>,
    origin: Option<String>,
    obtained: Option<NaiveDate>,
    notes: Option<Vec<String>>,
}

impl NewPlant {
    pub fn new(date_format: &str) -> NewPlant {
        NewPlant {
            current_step: Step::PlantName,
            date_format: date_format.to_owned(),
            plant_name: None,
            species_name: None,
            height: None,
            width: None,
            health: None,
            location: None,
            autowatered: None,
            origin: None,
            obtained: None,
            notes: None,
        }
    }
}

impl Default for NewPlant {
    fn default() -> Self {
        NewPlant::new("%d.%m.%Y")
    }
}

impl Action for NewPlant {
    fn handle_input<T: DatabaseManager>(
        &mut self,
        input: String,
        db_man: &mut T,
    ) -> Result<(), Error> {
        match self.current_step {
            Step::PlantName => {
                let name = input.trim().to_owned();
                let exists = db_man.plant_exists(&name)?;
                if exists {
                    Err(Error::PlantExists(name))
                } else {
                    self.plant_name = Some(name);
                    self.current_step = Step::SpeciesName;
                    Ok(())
                }
            }
            Step::SpeciesName => {
                let name = input_species(input, db_man)?;
                self.species_name = Some(name);
                self.current_step = Step::Height;
                Ok(())
            }
            Step::Height => {
                let height = input
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| Error::ParseError("Height".to_owned()))?;
                self.height = Some(height);
                self.current_step = Step::Width;
                Ok(())
            }
            Step::Width => {
                let width = input
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
                self.current_step = Step::Location;
                Ok(())
            }
            Step::Location => {
                match db_man.get_location(input.trim()) {
                    Ok(loc) => self.location = Some(PlantLocation::Location(Box::new(loc))),
                    Err(_) => {
                        log::warn!("Could not find location {input}");
                        self.location = Some(PlantLocation::Other(input.trim().to_owned()))
                    }
                }
                self.current_step = Step::AutoWatered;
                Ok(())
            }
            Step::AutoWatered => {
                let is_autowatered = match input.trim().to_lowercase().as_str() {
                    "y" => Ok(true),
                    "n" => Ok(false),
                    _ => Err(Error::ParseError("bool".to_owned())),
                }?;
                self.autowatered = Some(is_autowatered);
                self.current_step = Step::Origin;
                Ok(())
            }
            Step::Origin => {
                self.origin = Some(input.trim().to_owned());
                self.current_step = Step::ObtainedDate;
                Ok(())
            }
            Step::ObtainedDate => {
                let date = NaiveDate::parse_from_str(input.trim(), &self.date_format)
                    .map_err(|_| Error::ParseError("Date".to_owned()))?;
                self.obtained = Some(date);
                self.current_step = Step::Notes;
                Ok(())
            }
            Step::Notes => {
                self.notes = if input.trim().to_lowercase() == "done" {
                    None
                } else {
                    Some(
                        input
                            .split(',')
                            .map(|st| st.trim().to_owned())
                            .collect::<Vec<String>>(),
                    )
                };
                self.current_step = Step::Done;
                Ok(())
            }
            Step::Done => Err(Error::ActionAlreadyDone("New Plant".to_owned())),
        }
    }

    fn is_done(&self) -> bool {
        self.current_step == Step::Done
    }

    fn write_result<T: DatabaseManager>(&self, db_man: &mut T) -> Result<String, Error> {
        let plant_name = self
            .plant_name
            .clone()
            .ok_or(Error::MissingInput("Plant Name".to_owned()))?;
        let auto_watering = self
            .autowatered
            .ok_or(Error::MissingInput("Autowatered".to_owned()))?;
        let current_location = self
            .location
            .clone()
            .ok_or(Error::MissingInput("Location".to_owned()))?;
        let obtained = self
            .obtained
            .ok_or(Error::MissingInput("Obtained Date".to_owned()))?;
        let origin = self
            .origin
            .clone()
            .ok_or(Error::MissingInput("Origin".to_owned()))?;
        let plant_notes = self.notes.clone().unwrap_or(vec![]);
        let species_name = self
            .species_name
            .clone()
            .ok_or(Error::MissingInput("Species Name".to_owned()))?;
        let species = match db_man.get_species(&species_name) {
            Ok(sp) => PlantSpecies::Species(Box::new(sp)),
            Err(_) => PlantSpecies::Other(species_name),
        };

        let height = self
            .height
            .ok_or(Error::MissingInput("Height".to_owned()))?;
        let width = self.width.ok_or(Error::MissingInput("Width".to_owned()))?;
        let health = self
            .health
            .ok_or(Error::MissingInput("Health".to_owned()))?;

        let plant_growth = GrowthItem {
            plant: plant_name.clone(),
            date: Local::now().date_naive(),
            height_cm: height,
            width_cm: width,
            note: Some("Created during plant creation".to_owned()),
            health,
        };

        let plant_json = PlantInfo {
            name: plant_name.clone(),
            auto_water: auto_watering,
            location: current_location,
            obtained,
            origin,
            notes: plant_notes,
            species,
        };

        db_man.write_plant(plant_json)?;
        db_man.write_growth(plant_growth)?;
        let ret_msg = format!("Successfully saved plant {plant_name}");
        Ok(ret_msg)
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        match self.current_step {
            Step::PlantName => Ok("Please enter plant name".to_owned()),
            Step::SpeciesName => Ok("Please enter species".to_owned()),
            Step::Height => Ok("Please enter height (cm)".to_owned()),
            Step::Width => Ok("Please enter width (cm)".to_owned()),
            Step::Health => Ok("Please enter health (0-5)".to_owned()),
            Step::Location => Ok("Please enter location".to_owned()),
            Step::AutoWatered => Ok("Is plant autowatered? (y/n)".to_owned()),
            Step::Origin => Ok("Please enter plant origin".to_owned()),
            Step::ObtainedDate => Ok(format!("Please enter obtained date ({})", self.date_format)),
            Step::Notes => Ok("Please enter notes (Enter \"Done\" for no notes)".to_owned()),
            Step::Done => Err(Error::ActionAlreadyDone("New Plant".to_owned())),
        }
    }
}

impl From<NewPlant> for BotAction {
    fn from(newp: NewPlant) -> BotAction {
        BotAction::NewPlant(newp)
    }
}

#[cfg(test)]
mod new_plant_tests {
    use super::{Action, BotAction, NewPlant, PlantLocation, Step};
    use crate::test_common::{example_date1, example_location, DummyManager};

    #[test]
    fn new_plant_default() {
        let result = NewPlant::default();
        let expected = NewPlant {
            current_step: Step::PlantName,
            date_format: "%d.%m.%Y".to_owned(),
            plant_name: None,
            species_name: None,
            height: None,
            width: None,
            health: None,
            location: None,
            autowatered: None,
            origin: None,
            obtained: None,
            notes: None,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plant() {
        let mut result = NewPlant::default();
        result
            .handle_input("NewPlant".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::SpeciesName;
        expected.plant_name = Some("NewPlant".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_plant_err() {
        let result = NewPlant::default().handle_input("Plant1".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_sp() {
        let mut result = NewPlant::default();
        result.current_step = Step::SpeciesName;
        result
            .handle_input("Species1".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::Height;
        expected.species_name = Some("Species1".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_species_err() {
        let mut action = NewPlant::default();
        action.current_step = Step::SpeciesName;
        let result = action.handle_input("not a species".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_height() {
        let mut result = NewPlant::default();
        result.current_step = Step::Height;
        result
            .handle_input("1.0".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::Width;
        expected.height = Some(1.0);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_height_err() {
        let mut action = NewPlant::default();
        action.current_step = Step::Height;
        let result = action.handle_input("Not a number".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_width() {
        let mut result = NewPlant::default();
        result.current_step = Step::Width;
        result
            .handle_input("1.0".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::Health;
        expected.width = Some(1.0);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_width_err() {
        let mut action = NewPlant::default();
        action.current_step = Step::Width;
        let result = action.handle_input("Not a number".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_health() {
        let mut result = NewPlant::default();
        result.current_step = Step::Health;
        result
            .handle_input("3".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::Location;
        expected.health = Some(3);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_health_err() {
        let mut action = NewPlant::default();
        action.current_step = Step::Health;
        let result = action.handle_input("6".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_location_loc() {
        let mut result = NewPlant::default();
        result.current_step = Step::Location;
        result
            .handle_input("Inside".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::AutoWatered;
        expected.location = Some(PlantLocation::Location(Box::new(example_location())));
        assert_eq!(result, expected)
    }

    #[test]
    fn input_location_other() {
        let mut result = NewPlant::default();
        result.current_step = Step::Location;
        result
            .handle_input("other location".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::AutoWatered;
        expected.location = Some(PlantLocation::Other("other location".to_owned()));
        assert_eq!(result, expected)
    }

    #[test]
    fn input_autowater() {
        let mut result = NewPlant::default();
        result.current_step = Step::AutoWatered;
        result
            .handle_input("y".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::Origin;
        expected.autowatered = Some(true);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_autowater_err() {
        let mut action = NewPlant::default();
        action.current_step = Step::AutoWatered;
        let result = action.handle_input("not a boolean".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_origin() {
        let mut result = NewPlant::default();
        result.current_step = Step::Origin;
        result
            .handle_input("test origin".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::ObtainedDate;
        expected.origin = Some("test origin".to_owned());
    }

    #[test]
    fn input_obtained() {
        let mut result = NewPlant::default();
        result.current_step = Step::ObtainedDate;
        result
            .handle_input("01.01.1970".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::Notes;
        expected.obtained = Some(example_date1());
        assert_eq!(result, expected)
    }

    #[test]
    fn input_obtained_err() {
        let mut action = NewPlant::default();
        action.current_step = Step::ObtainedDate;
        let result = action.handle_input("not a date".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn input_notes_some() {
        let mut result = NewPlant::default();
        result.current_step = Step::Notes;
        result
            .handle_input("note1,note2".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::Done;
        expected.notes = Some(vec!["note1".to_owned(), "note2".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn input_notes_none() {
        let mut result = NewPlant::default();
        result.current_step = Step::Notes;
        result
            .handle_input("Done".to_owned(), &mut DummyManager {})
            .unwrap();
        let mut expected = NewPlant::default();
        expected.current_step = Step::Done;
        expected.notes = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn input_err() {
        let mut action = NewPlant::default();
        action.current_step = Step::Done;
        let result = action.handle_input("".to_owned(), &mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn done_done() {
        let mut action = NewPlant::default();
        action.current_step = Step::Done;
        assert!(action.is_done())
    }

    #[test]
    fn done_notdone() {
        assert!(!NewPlant::default().is_done())
    }

    #[test]
    fn write_no_plant() {
        let result = NewPlant::default().write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_species() {
        let mut action = NewPlant::default();
        action.plant_name = Some("Plant1".to_owned());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_height() {
        let mut action = NewPlant::default();
        action.plant_name = Some("Plant1".to_owned());
        action.species_name = Some("SPecies1".to_owned());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_width() {
        let mut action = NewPlant::default();
        action.plant_name = Some("Plant1".to_owned());
        action.species_name = Some("SPecies1".to_owned());
        action.height = Some(1.0);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn wirte_no_health() {
        let mut action = NewPlant::default();
        action.plant_name = Some("Plant1".to_owned());
        action.species_name = Some("SPecies1".to_owned());
        action.height = Some(1.0);
        action.width = Some(1.0);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_location() {
        let mut action = NewPlant::default();
        action.plant_name = Some("Plant1".to_owned());
        action.species_name = Some("SPecies1".to_owned());
        action.height = Some(1.0);
        action.width = Some(1.0);
        action.health = Some(3);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_autowater() {
        let mut action = NewPlant::default();
        action.plant_name = Some("Plant1".to_owned());
        action.species_name = Some("SPecies1".to_owned());
        action.height = Some(1.0);
        action.width = Some(1.0);
        action.health = Some(3);
        action.location = Some(PlantLocation::Other("location".to_owned()));
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_origin() {
        let mut action = NewPlant::default();
        action.plant_name = Some("Plant1".to_owned());
        action.species_name = Some("SPecies1".to_owned());
        action.height = Some(1.0);
        action.width = Some(1.0);
        action.health = Some(3);
        action.location = Some(PlantLocation::Other("location".to_owned()));
        action.autowatered = Some(true);
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write_no_obtained() {
        let mut action = NewPlant::default();
        action.plant_name = Some("Plant1".to_owned());
        action.species_name = Some("SPecies1".to_owned());
        action.height = Some(1.0);
        action.width = Some(1.0);
        action.health = Some(3);
        action.location = Some(PlantLocation::Other("location".to_owned()));
        action.autowatered = Some(true);
        action.origin = Some("origin".to_owned());
        let result = action.write_result(&mut DummyManager {});
        assert!(result.is_err())
    }

    #[test]
    fn write() {
        let mut action = NewPlant::default();
        action.plant_name = Some("Plant1".to_owned());
        action.species_name = Some("SPecies1".to_owned());
        action.height = Some(1.0);
        action.width = Some(1.0);
        action.health = Some(3);
        action.location = Some(PlantLocation::Other("location".to_owned()));
        action.autowatered = Some(true);
        action.origin = Some("origin".to_owned());
        action.obtained = Some(example_date1());
        let result = action.write_result(&mut DummyManager {});
        println!("{result:?}");
        assert!(result.is_ok())
    }

    #[test]
    fn next_name() {
        let result = NewPlant::default().get_next_prompt().unwrap();
        let expected = "Please enter plant name";
        assert_eq!(result, expected);
    }

    #[test]
    fn next_species() {
        let mut action = NewPlant::default();
        action.current_step = Step::SpeciesName;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter species";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_height() {
        let mut action = NewPlant::default();
        action.current_step = Step::Height;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter height (cm)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_width() {
        let mut action = NewPlant::default();
        action.current_step = Step::Width;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter width (cm)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_health() {
        let mut action = NewPlant::default();
        action.current_step = Step::Health;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter health (0-5)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_location() {
        let mut action = NewPlant::default();
        action.current_step = Step::Location;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter location";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_autowater() {
        let mut action = NewPlant::default();
        action.current_step = Step::AutoWatered;
        let result = action.get_next_prompt().unwrap();
        let expected = "Is plant autowatered? (y/n)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_origin() {
        let mut action = NewPlant::default();
        action.current_step = Step::Origin;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter plant origin";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_obtained() {
        let mut action = NewPlant::default();
        action.current_step = Step::ObtainedDate;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter obtained date (%d.%m.%Y)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_notes() {
        let mut action = NewPlant::default();
        action.current_step = Step::Notes;
        let result = action.get_next_prompt().unwrap();
        let expected = "Please enter notes (Enter \"Done\" for no notes)";
        assert_eq!(result, expected)
    }

    #[test]
    fn next_err() {
        let mut action = NewPlant::default();
        action.current_step = Step::Done;
        let result = action.get_next_prompt();
        assert!(result.is_err())
    }

    #[test]
    fn into_botaction() {
        let result = <NewPlant as Into<BotAction>>::into(NewPlant::default());
        let expected = BotAction::NewPlant(NewPlant::default());
        assert_eq!(result, expected)
    }
}
