use super::growth_item::GrowthItem;
use super::log_item::LogItem;
use super::species::Species;
use datetime::convenience::Today;
use datetime::LocalDate;

type PlantImage = (LocalDate, String);

pub struct Plant {
    name: String,
    species: Species,
    location: String,
    origin: String,
    obtained: LocalDate,
    auto_water: bool,
    notes: Vec<String>,
    images: Vec<PlantImage>,
    activities: Vec<LogItem>,
    growth: Vec<GrowthItem>,
    next_watering: Option<LocalDate>,
    next_fertilizing: Option<LocalDate>,
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

    fn get_next_watering(&self) -> Option<LocalDate> {
        Some(LocalDate::today())
    }

    fn get_next_fertilizing(&self) -> Option<LocalDate> {
        Some(LocalDate::today())
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
