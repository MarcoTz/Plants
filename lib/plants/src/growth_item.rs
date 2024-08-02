use chrono::NaiveDate;

pub struct GrowthItem {
    date: NaiveDate,
    height_cm: f32,
    width_cm: f32,
    note: String,
    health: i32,
}
