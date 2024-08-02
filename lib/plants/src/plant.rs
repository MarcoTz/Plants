use super::growth_item::GrowthItem;
use super::log_item::LogItem;
use chrono::{Local, NaiveDate};

type PlantImage = (NaiveDate, String);

pub struct Plant {
    pub name: String,
    pub species_name: String,
    pub location: String,
    pub origin: String,
    pub obtained: NaiveDate,
    pub auto_water: bool,
    pub notes: Vec<String>,
    pub images: Vec<PlantImage>,
    pub activities: Vec<LogItem>,
    pub growth: Vec<GrowthItem>,
}

impl Plant {
    fn get_watering_activities(&self) -> Vec<LogItem> {
        vec![]
    }

    fn get_fertilizing_activities(&self) -> Vec<LogItem> {
        vec![]
    }

    fn get_age_days(&self) -> i32 {
        1
    }

    fn get_next_watering(&self) -> Option<NaiveDate> {
        Some(Local::now().date_naive())
    }

    fn get_next_fertilizing(&self) -> Option<NaiveDate> {
        Some(Local::now().date_naive())
    }

    fn get_height(&self) -> f32 {
        1.0
    }

    fn get_width(&self) -> f32 {
        1.0
    }

    fn get_health(&self) -> i32 {
        3
    }

    fn get_watering_frequency(&self) -> f32 {
        1.0
    }

    fn get_fertilizing_frequency(&self) -> f32 {
        1.0
    }

    fn get_growth_speed(&self) -> f32 {
        1.0
    }

    fn add_activity(&mut self, new_activity: LogItem) {
        return;
    }

    fn add_growth(&mut self, new_growth: GrowthItem) {
        return;
    }

    fn add_image(&mut self, new_image: PlantImage) {
        return;
    }
}
