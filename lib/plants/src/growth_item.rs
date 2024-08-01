use datetime::LocalDate;

pub struct GrowthItem {
    date: LocalDate,
    height_cm: f32,
    width_cm: f32,
    note: String,
    health: i32,
}
