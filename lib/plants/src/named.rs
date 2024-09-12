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

#[cfg(test)]
mod named_tests {
    use super::Named;
    use crate::test_common::{empty_plant, example_location, example_plant, example_species};

    #[test]
    fn plant_name() {
        let result = example_plant().get_name();
        let expected = "A Plant";
        assert_eq!(result, expected)
    }

    #[test]
    fn info_name() {
        let result = example_plant().info.get_name();
        let expected = "A Plant";
        assert_eq!(result, expected)
    }

    #[test]
    fn plant_species_name_species() {
        let result = example_plant().info.species.get_name();
        let expected = "Test species";
        assert_eq!(result, expected)
    }

    #[test]
    fn plant_species_name_other() {
        let result = empty_plant().info.species.get_name();
        let expected = "another species";
        assert_eq!(result, expected)
    }

    #[test]
    fn plant_location_name_location() {
        let result = example_plant().info.location.get_name();
        let expected = "Inside";
        assert_eq!(result, expected)
    }

    #[test]
    fn platn_location_name_other() {
        let result = empty_plant().info.location.get_name();
        let expected = "another location";
        assert_eq!(result, expected)
    }

    #[test]
    fn location_name() {
        let result = example_location().get_name();
        let expected = "Inside";
        assert_eq!(result, expected)
    }

    #[test]
    fn species_name() {
        let result = example_species().get_name();
        let expected = "Test species";
        assert_eq!(result, expected)
    }
}
