use crate::location::Location;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PlantLocation {
    Location(Box<Location>),
    Other(String),
}

impl From<Location> for PlantLocation {
    fn from(loc: Location) -> PlantLocation {
        PlantLocation::Location(Box::new(loc))
    }
}

impl fmt::Display for PlantLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlantLocation::Location(loc) => f.write_str(&loc.name),
            PlantLocation::Other(loc) => f.write_str(loc),
        }
    }
}
