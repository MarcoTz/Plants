use super::super::errors::Error;
use super::page_component::PageComponent;
use html::{
    attribute::Attribute,
    div::Div,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
    img::Img,
    link::Link,
};
use plants::plant::Plant;
use std::collections::HashSet;
use std::rc::Rc;

pub struct PlantListItem {
    plant_url: String,
    plant_name: String,
    plant_preview_url: String,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    species_url: Option<String>,
    species_name: Option<String>,
}
pub struct LocationGroup {
    location: String,
    plant_items: Vec<PlantListItem>,
}
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
            attributes: vec![Attribute::Id("plant_list".to_owned())],
            content: Rc::new(location_divs.into()),
        }
        .into()
    }
}
impl PageComponent for LocationGroup {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut plant_divs = vec![Headline {
            size: HeaderSize::H2,
            content: Rc::new(self.location.clone().into()),
        }
        .into()];
        for plant_item in self.plant_items.iter() {
            plant_divs.push(plant_item.render(date_format));
        }
        Div {
            attributes: vec![Attribute::Class("location_group".to_owned())],
            content: Rc::new(plant_divs.into()),
        }
        .into()
    }
}

impl PageComponent for PlantListItem {
    fn render(&self, _: &str) -> HtmlElement {
        let mut div_content = vec![
            Link {
                attributes: vec![
                    Attribute::Href(self.plant_url.clone()),
                    Attribute::Class("plant_link".to_owned()),
                ],
                content: Rc::new(self.plant_name.clone().into()),
            }
            .into(),
            HtmlElement::Br,
        ];
        match (self.species_url.clone(), self.species_name.clone()) {
            (None, _) => (),
            (_, None) => (),
            (Some(url), Some(name)) => div_content.push(
                Div {
                    attributes: vec![Attribute::Class("species_link".to_owned())],
                    content: Rc::new(
                        Link {
                            attributes: vec![Attribute::Href(url.clone())],
                            content: Rc::new(name.clone().into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }

        div_content.push(
            vec![
                HtmlElement::Br,
                Img {
                    attributes: vec![
                        Attribute::Style("cursor:default;".to_owned()),
                        Attribute::Id("plant_overview".to_owned()),
                        Attribute::Src(self.plant_preview_url.clone()),
                    ],
                }
                .into(),
            ]
            .into(),
        );

        match self.temp_max {
            None => (),
            Some(temp) => div_content.push(
                Div {
                    attributes: vec![Attribute::Class("temp_max".to_owned())],
                    content: Rc::new(temp.to_string().into()),
                }
                .into(),
            ),
        }

        match self.temp_min {
            None => (),
            Some(temp) => div_content.push(
                Div {
                    attributes: vec![Attribute::Class("temp_min".to_owned())],
                    content: Rc::new(temp.to_string().into()),
                }
                .into(),
            ),
        }

        Div {
            attributes: vec![Attribute::Class("plant_list_item".to_owned())],
            content: Rc::new(div_content.into()),
        }
        .into()
    }
}

impl From<&Plant> for PlantListItem {
    fn from(plant: &Plant) -> PlantListItem {
        PlantListItem {
            plant_url: plant.get_url("plants/"),
            plant_name: plant.name.clone(),
            plant_preview_url: "".to_owned(),
            temp_max: plant.species.as_ref().map(|x| x.temp_max),
            temp_min: plant.species.as_ref().map(|x| x.temp_min),
            species_url: plant.species.as_ref().map(|x| x.get_url("species/")),
            species_name: plant.species.as_ref().map(|x| x.name.clone()),
        }
    }
}

impl TryFrom<&[&Plant]> for LocationGroup {
    type Error = Error;
    fn try_from(plants: &[&Plant]) -> Result<LocationGroup, Self::Error> {
        let mut locations = HashSet::new();
        for plant in plants.iter() {
            locations.insert(plant.location.clone());
        }
        let locations_vec: Vec<&String> = locations.iter().collect();
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
            Ok(LocationGroup {
                location: location.clone().clone(),
                plant_items: plants.iter().cloned().map(|p| p.into()).collect(),
            })
        }
    }
}
