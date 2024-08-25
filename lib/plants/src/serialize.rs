pub mod date_serializer {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%d.%m.%Y";

    pub fn serialize<S: Serializer>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
        let s = String::deserialize(deserializer)?;
        let d = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(d)
    }
}

pub mod species_serializer {
    use crate::{named::Named, plant::PlantSpecies};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(
        species: &PlantSpecies,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let s = species.get_name().to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<PlantSpecies, D::Error> {
        let s = String::deserialize(deserializer)?;
        let sp = PlantSpecies::Other(s);
        Ok(sp)
    }
}
