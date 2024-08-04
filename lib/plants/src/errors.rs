use std::fmt;

pub enum PlantError {
    SunlightError(String),
    GrowthError(String),
}

impl fmt::Debug for PlantError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlantError::SunlightError(msg) => frmt.write_str(msg),
            PlantError::GrowthError(plant_name) => {
                frmt.write_str(&format!("Could not find growth for plant {}", plant_name))
            }
        }
    }
}
