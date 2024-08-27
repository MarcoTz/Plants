use super::{
    location::Location,
    plant::{Plant, PlantInfo, PlantLocation, PlantSpecies},
    species::Species,
};

pub trait Named {
    fn get_name(&self) -> String;
}

impl Named for Plant {
    fn get_name(&self) -> String {
        self.info.name.clone()
    }
}

impl Named for PlantInfo {
    fn get_name(&self) -> String {
        self.name.clone()
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
impl Named for PlantLocation {
    fn get_name(&self) -> String {
        match self {
            PlantLocation::Location(loc) => loc.get_name(),
            PlantLocation::Other(st) => st.clone(),
        }
    }
}

impl Named for Location {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Named for Species {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
