use super::plant::{Plant, PlantSpecies};
use super::species::Species;

pub trait Named {
    fn get_name(&self) -> String;
}

impl Named for Plant {
    fn get_name(&self) -> String {
        self.info.name.clone()
    }
}

impl Named for PlantSpecies {
    fn get_name(&self) -> String {
        match self {
            PlantSpecies::Other(name) => name.clone(),
            PlantSpecies::Species(sp) => sp.get_name(),
        }
    }
}

impl Named for Species {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
