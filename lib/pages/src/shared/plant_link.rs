use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{HtmlElement, A},
};
use plants::plant::Plant;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlantLink {
    pub plant_name: String,
    plant_url: String,
}

impl PageComponent for PlantLink {
    fn render(&self, _: &str) -> HtmlElement {
        A {
            attributes: vec![
                Attribute::Href(self.plant_url.clone()),
                Attribute::Class(vec!["plant_link".to_owned()]),
            ],
            content: Rc::new(self.plant_name.clone().into()),
        }
        .into()
    }
}

impl From<(&Plant, &str)> for PlantLink {
    fn from((plant, plant_base): (&Plant, &str)) -> PlantLink {
        PlantLink {
            plant_name: plant.info.name.clone(),
            plant_url: plant.get_url(plant_base),
        }
    }
}
