use crate::{
    errors::Error,
    page::PageComponent,
    shared::{plant_link::PlantLink, species_link::SpeciesLink},
};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement, Img},
};
use plants::{
    named::Named,
    plant::{Plant, PlantSpecies},
};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct PlantListItem {
    plant_link: PlantLink,
    plant_preview_url: Option<String>,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    species_link: Option<SpeciesLink>,
    location: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocationGroup {
    location: String,
    plant_items: Vec<PlantListItem>,
}

#[derive(Debug, PartialEq)]
pub struct PlantList {
    locations: Vec<LocationGroup>,
}

impl PageComponent for PlantList {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut location_divs = vec![];
        for location in self.locations.iter() {
            location_divs.push(location.render(date_format));
        }
        Div {
            attributes: vec![
                Attribute::Id("plant_list".to_owned()),
                Attribute::Class(vec!["flex_container".to_owned()]),
            ],
            content: Rc::new(location_divs.into()),
        }
        .into()
    }
}
impl PageComponent for LocationGroup {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut plant_divs = vec![Headline {
            attributes: vec![
                Attribute::Class(vec!["location_header".to_owned()]),
                Attribute::Id(self.location.clone()),
            ],
            size: HeaderSize::H2,
            content: Rc::new(self.location.clone().into()),
        }
        .into()];
        let mut plant_items_ordered = self.plant_items.clone();
        plant_items_ordered
            .sort_by(|it1, it2| it1.plant_link.plant_name.cmp(&it2.plant_link.plant_name));
        for plant_item in plant_items_ordered.iter() {
            plant_divs.push(plant_item.render(date_format));
        }
        Div {
            attributes: vec![Attribute::Class(vec![
                "location_group".to_owned(),
                "flex_container".to_owned(),
                "alternating_children".to_owned(),
            ])],
            content: Rc::new(plant_divs.into()),
        }
        .into()
    }
}

impl PageComponent for PlantListItem {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut div_content = vec![self.plant_link.render(date_format), HtmlElement::Br];
        match self.species_link.as_ref() {
            None => (),
            Some(link) => {
                div_content.push(
                    Div {
                        attributes: vec![Attribute::Class(vec!["species_link".to_owned()])],
                        content: Rc::new(link.render(date_format)),
                    }
                    .into(),
                );
            }
        }

        match self.plant_preview_url.clone() {
            None => (),
            Some(url) => {
                div_content.push(
                    vec![
                        HtmlElement::Br,
                        Img {
                            attributes: vec![
                                Attribute::Style("cursor:default;".to_owned()),
                                Attribute::Class(vec!["plant_preview".to_owned()]),
                                Attribute::Src(url),
                            ],
                        }
                        .into(),
                    ]
                    .into(),
                );
            }
        }

        match self.temp_max {
            None => (),
            Some(temp) => div_content.push(
                Div {
                    attributes: vec![Attribute::Class(vec!["temp_max".to_owned()])],
                    content: Rc::new(temp.to_string().into()),
                }
                .into(),
            ),
        }

        match self.temp_min {
            None => (),
            Some(temp) => div_content.push(
                Div {
                    attributes: vec![Attribute::Class(vec!["temp_min".to_owned()])],
                    content: Rc::new(temp.to_string().into()),
                }
                .into(),
            ),
        }

        div_content.push(
            Div {
                attributes: vec![Attribute::Class(vec!["location_name".to_owned()])],
                content: Rc::new(self.location.clone().into()),
            }
            .into(),
        );

        Div {
            attributes: vec![Attribute::Class(vec!["plant_list_item".to_owned()])],
            content: Rc::new(div_content.into()),
        }
        .into()
    }
}

impl From<&Plant> for PlantListItem {
    fn from(plant: &Plant) -> PlantListItem {
        let img_base = "img/".to_owned();
        let sp = match &plant.info.species {
            PlantSpecies::Other(_) => None,
            PlantSpecies::Species(sp) => Some(sp),
        };
        PlantListItem {
            plant_link: (plant, "plants").into(),
            plant_preview_url: plant.get_preview_image_url(&img_base),
            temp_max: sp.map(|sp| sp.temp_max),
            temp_min: sp.map(|sp| sp.temp_min),
            species_link: sp.map(|sp| (sp.as_ref(), "species").into()),
            location: plant.info.location.get_name(),
        }
    }
}

impl TryFrom<&[Plant]> for LocationGroup {
    type Error = Error;
    fn try_from(plants: &[Plant]) -> Result<LocationGroup, Self::Error> {
        let mut locations = HashSet::new();
        for plant in plants.iter() {
            locations.insert(plant.info.location.get_name());
        }
        let locations_vec: Vec<String> = locations.iter().cloned().collect();
        if locations_vec.len() != 1 {
            Err(Error::LocationError(
                locations_vec
                    .iter()
                    .cloned()
                    .map(|s| s.to_owned())
                    .collect(),
            ))
        } else {
            let location = locations_vec.first().ok_or(Error::LocationError(
                locations_vec
                    .iter()
                    .cloned()
                    .map(|s| s.to_owned())
                    .collect(),
            ))?;
            let mut plant_items: Vec<PlantListItem> =
                plants.iter().cloned().map(|p| (&p).into()).collect();
            plant_items
                .sort_by(|it1, it2| it1.plant_link.plant_name.cmp(&it2.plant_link.plant_name));
            Ok(LocationGroup {
                location: location.clone(),
                plant_items,
            })
        }
    }
}

impl From<&[Plant]> for PlantList {
    fn from(plants: &[Plant]) -> PlantList {
        log::info!("Loading Plant List");
        let mut by_location: HashMap<String, Vec<Plant>> = HashMap::new();
        for plant in plants.iter() {
            match by_location.get_mut(&plant.info.location.get_name()) {
                None => {
                    by_location.insert(plant.info.location.get_name(), vec![plant.clone()]);
                }
                Some(location_vec) => {
                    location_vec.push(plant.clone());
                }
            };
        }

        let mut location_groups = by_location
            .values()
            .map(|plants| plants.as_slice().try_into())
            .collect::<Result<Vec<LocationGroup>, Error>>()
            .unwrap();
        location_groups.sort_by(|loc1, loc2| loc1.location.cmp(&loc2.location));

        PlantList {
            locations: location_groups,
        }
    }
}

#[cfg(test)]
mod plant_list_tests {
    use super::{LocationGroup, PlantList};
    use crate::test_common::{example_plant1, example_plant2};
    use plants::plant::PlantLocation;

    #[test]
    fn location_from_fail() {
        let mut plant1 = example_plant1();
        plant1.info.location = PlantLocation::Other("anoter location".to_owned());
        let result = LocationGroup::try_from(vec![plant1, example_plant2()].as_slice());
        assert!(result.is_err())
    }

    #[test]
    fn list_from_multiple_loc() {
        let mut plant2 = example_plant2();
        plant2.info.location = PlantLocation::Other("another location".to_owned());
        let result = PlantList::from(vec![example_plant1(), plant2.clone()].as_slice());
        let expected = PlantList {
            locations: vec![
                LocationGroup::try_from(vec![plant2].as_slice()).unwrap(),
                LocationGroup::try_from(vec![example_plant1()].as_slice()).unwrap(),
            ],
        };
        assert_eq!(result, expected)
    }
}
