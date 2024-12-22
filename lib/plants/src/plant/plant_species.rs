use crate::{errors::Error, species::Species};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum PlantSpecies {
    Species(Box<Species>),
    Other(String),
}
impl TryFrom<PlantSpecies> for Species {
    type Error = Error;
    fn try_from(pl_sp: PlantSpecies) -> Result<Species, Self::Error> {
        match pl_sp {
            PlantSpecies::Species(sp) => Ok(*sp),
            PlantSpecies::Other(sp) => Err(Error::SpeciesNotFound(sp)),
        }
    }
}

impl From<Species> for PlantSpecies {
    fn from(sp: Species) -> PlantSpecies {
        PlantSpecies::Species(Box::new(sp))
    }
}

impl fmt::Display for PlantSpecies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlantSpecies::Species(sp) => f.write_str(&sp.name),
            PlantSpecies::Other(sp) => f.write_str(sp),
        }
    }
}
