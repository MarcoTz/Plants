use super::{input_handlers::input_species, Action, BotAction, DatabaseManager, Error};

#[derive(Debug, PartialEq, Clone)]
pub struct GetSpeciesDetails {
    species_name: Option<String>,
    done: bool,
}

impl Default for GetSpeciesDetails {
    fn default() -> GetSpeciesDetails {
        GetSpeciesDetails {
            species_name: None,
            done: false,
        }
    }
}

impl Action for GetSpeciesDetails {
    fn handle_input<T>(&mut self, input: String, db_man: &mut T) -> Result<(), Error>
    where
        T: DatabaseManager,
    {
        let species_name = input_species(input, db_man)?;
        self.species_name = Some(species_name);
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
        let species_name = self
            .species_name
            .clone()
            .ok_or(Error::MissingInput("Species to look up".to_owned()))?;
        let species = db_man.get_species(&species_name)?;
        let plant_dist_str = if let Some(dist) = species.planting_distance {
            format!("\n\tPlanting Distance: {dist}cm")
        } else {
            "".to_owned()
        };
        let avg_water_str = if let Some(days) = species.avg_watering_days {
            format!("\n\tAverage Days between Watering: {days} days")
        } else {
            "".to_owned()
        };
        let avg_fert_str = if let Some(days) = species.avg_fertilizing_days {
            format!("\n\tAverage Days between Fertilizing: {days} days")
        } else {
            "".to_owned()
        };
        Ok(format!(
            "{} ({})\n\tGenus: {}\n\tFamily: {}\n\tSunlight Requirements: {}\n\tTemperature Range (Survivable) : {}-{}\n\tTemperature Range (Optimal):{}-{}\n\tpH Range: {}-{},{}\n\tWatering Notes: {}{}\n\tFertilizing Notes: {}{}\n\tPruning Notes: {}\n\tCompanion Plants: {}\n\tAdditional Notes: {}",
            species.name,
            species.scientific_name,
            species.genus,
            species.family,
            species.sunlight,
            species.temp_min,
            species.temp_max,
            species.opt_temp_min,
            species.opt_temp_max,
            species.ph_min,
            species.ph_max,
            plant_dist_str,
            species.watering_notes.join(", "),
            avg_water_str,
            species.fertilizing_notes.join(", "),
            avg_fert_str,
            species.pruning_notes.join(", "),
            species.companions.join(", "),
            species.additional_notes.join(", ")

        ))
    }

    fn get_next_prompt(&self) -> Result<String, Error> {
        if self.done {
            Err(Error::ActionAlreadyDone("Get Species Details".to_owned()))
        } else {
            Ok("Please enter species to look up".to_owned())
        }
    }
}

impl From<GetSpeciesDetails> for BotAction {
    fn from(gt: GetSpeciesDetails) -> BotAction {
        BotAction::GetSpeciesDetails(gt)
    }
}
