use super::super::{
    super::html::{
        attribute::Attribute,
        div::Div,
        headline::{HeaderSize, Headline},
        html_element::HtmlElement,
        img::Img,
        link::Link,
    },
    page::PageComponent,
};
use std::rc::Rc;

pub struct PlantListItem {
    plant_url: String,
    plant_name: String,
    plant_preview_url: String,
    temp_max: i32,
    temp_min: i32,
    species_url: String,
    species_name: String,
}
pub struct LocationGroup {
    location: String,
    plant_items: Vec<PlantListItem>,
}
pub struct PlantList {
    locations: Vec<LocationGroup>,
}
impl PageComponent for PlantList {
    fn render(&self) -> HtmlElement {
        let mut location_divs = vec![];
        for location in self.locations.iter() {
            location_divs.push(location.render());
        }
        Div {
            attributes: vec![Attribute::Id("plant_list".to_owned())],
            content: Rc::new(location_divs.into()),
        }
        .into()
    }
}
impl PageComponent for LocationGroup {
    fn render(&self) -> HtmlElement {
        let mut plant_divs = vec![Headline {
            size: HeaderSize::H2,
            content: Rc::new(self.location.clone().into()),
        }
        .into()];
        for plant_item in self.plant_items.iter() {
            plant_divs.push(plant_item.render());
        }
        Div {
            attributes: vec![Attribute::Class("location_group".to_owned())],
            content: Rc::new(plant_divs.into()),
        }
        .into()
    }
}

impl PageComponent for PlantListItem {
    fn render(&self) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Class("plant_list_item".to_owned())],
            content: Rc::new(
                vec![
                    Link {
                        attributes: vec![
                            Attribute::Href(self.plant_url.clone()),
                            Attribute::Class("plant_link".to_owned()),
                        ],
                        content: Rc::new(self.plant_name.clone().into()),
                    }
                    .into(),
                    HtmlElement::Br,
                    Div {
                        attributes: vec![Attribute::Class("species_link".to_owned())],
                        content: Rc::new(
                            Link {
                                attributes: vec![Attribute::Href(self.species_url.clone())],
                                content: Rc::new(self.species_name.clone().into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                    HtmlElement::Br,
                    Img {
                        attributes: vec![
                            Attribute::Style("cursor:default;".to_owned()),
                            Attribute::Id("plant_overview".to_owned()),
                            Attribute::Src(self.plant_preview_url.clone()),
                        ],
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class("temp_max".to_owned())],
                        content: Rc::new(self.temp_max.to_string().into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class("temp_min".to_owned())],
                        content: Rc::new(self.temp_min.to_string().into()),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
