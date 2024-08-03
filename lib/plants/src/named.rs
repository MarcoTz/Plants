use super::plant::Plant;
use super::species::Species;

pub trait Named {
    fn get_name(&self) -> String;
}

impl Named for Plant {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Named for Species {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
