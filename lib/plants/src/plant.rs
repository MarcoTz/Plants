use super::date::date_serializer;
use super::errors::Error;
use super::growth_item::GrowthItem;
use super::log_item::LogItem;
use super::species::Species;
use chrono::{Local, NaiveDate};
use serde::Serialize;

pub type PlantImage = (NaiveDate, String);

#[derive(Serialize)]
pub struct Plant {
    pub name: String,
    pub species_name: String,
    pub location: String,
    pub origin: String,
    #[serde(with = "date_serializer")]
    pub obtained: NaiveDate,
    pub auto_water: bool,
    pub notes: Vec<String>,
}

impl Plant {
    fn get_activities(&self, activity_name: &str, activities: &[LogItem]) -> Vec<LogItem> {
        let mut watering_activities = vec![];
        for activity in activities.iter() {
            if activity.activity.to_lowercase().trim() == activity_name.to_lowercase().trim()
                && activity.plant == self.name
            {
                watering_activities.push(activity.clone())
            }
        }
        watering_activities
    }

    fn get_watering_activities(&self, activities: &[LogItem]) -> Vec<LogItem> {
        self.get_activities("watering", activities)
    }

    fn get_fertilizing_activities(&self, activities: &[LogItem]) -> Vec<LogItem> {
        self.get_activities("fertilizing", activities)
    }

    fn get_age_days(&self) -> i64 {
        let today = Local::now().date_naive();
        let time_diff = today - self.obtained;
        time_diff.num_days()
    }

    fn get_next_activity_date(
        &self,
        activity_name: &str,
        activities: &[LogItem],
        species: &Species,
    ) -> Option<NaiveDate> {
        let self_activities = self.get_activities(activity_name, activities);
        let m_last_activity = self_activities.iter().max();
        match m_last_activity {
            None => Some(Local::now().date_naive()),
            Some(last_activity) => {
                let activity_delta = species.get_activity_delta(activity_name)?;
                Some(last_activity.date + activity_delta)
            }
        }
    }

    fn get_next_watering(&self, activities: &[LogItem], species: &Species) -> Option<NaiveDate> {
        self.get_next_activity_date("watering", activities, species)
    }

    fn get_next_fertilizing(&self, activities: &[LogItem], species: &Species) -> Option<NaiveDate> {
        self.get_next_activity_date("fertilizing", activities, species)
    }

    fn get_last_growth(&self, growth: &[GrowthItem]) -> Result<GrowthItem, Error> {
        let mut self_growth = vec![];
        for growth_item in growth.iter() {
            if growth_item.plant == self.name {
                self_growth.push(growth_item.clone());
            }
        }
        let last_growth = self_growth
            .iter()
            .max()
            .ok_or(Error::GrowthError(self.name.clone()))?;
        Ok(last_growth.clone())
    }
    fn get_height(&self, growth: &[GrowthItem]) -> Result<f32, Error> {
        let last_growth = self.get_last_growth(growth)?;
        Ok(last_growth.height_cm)
    }

    fn get_width(&self, growth: &[GrowthItem]) -> Result<f32, Error> {
        let last_growth = self.get_last_growth(growth)?;
        Ok(last_growth.width_cm)
    }

    fn get_health(&self, growth: &[GrowthItem]) -> Result<i32, Error> {
        let last_growth = self.get_last_growth(growth)?;
        Ok(last_growth.health)
    }

    fn get_activity_frequency(&self, activity_name: &str, activities: &[LogItem]) -> Option<f32> {
        let self_activities = self.get_activities(activity_name, activities);
        let first_activity = self_activities.iter().min()?;
        let last_activity = self_activities.iter().max()?;
        let activity_diff = last_activity.date - first_activity.date;
        match self_activities.len() {
            0 => None,
            n => Some(activity_diff.num_days() as f32 / n as f32),
        }
    }

    fn get_fertilizing_frequency(&self, activities: &[LogItem]) -> Option<f32> {
        self.get_activity_frequency("fertilizing", activities)
    }

    fn get_watering_frequency(&self, activities: &[LogItem]) -> Option<f32> {
        self.get_activity_frequency("watering", activities)
    }

    fn get_growth_speed(&self, growth: &[GrowthItem]) -> Option<(f32, f32)> {
        let self_growth = growth.iter().filter(|it| it.plant == self.name).cloned();
        let first_growth = self_growth.clone().min()?;
        let last_growth = self_growth.max()?;
        let height_diff = last_growth.height_cm - first_growth.height_cm;
        let width_diff = last_growth.width_cm - first_growth.width_cm;
        let time_diff = (last_growth.date - first_growth.date).num_days() as f32;
        if time_diff == 0.0 {
            None
        } else {
            Some((height_diff / time_diff, width_diff / time_diff))
        }
    }
}
