use crate::components::page_component::PageComponent;
use html::{a::A, attribute::Attribute, html_element::HtmlElement};
use plants::plant::Plant;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct PlantLink {
    plant_name: String,
    plant_url: String,
}

impl PageComponent for PlantLink {
    fn render(&self, _: &str) -> HtmlElement {
        A {
            attributes: vec![Attribute::Href(self.plant_url.clone())],
            content: Rc::new(self.plant_name.clone().into()),
        }
        .into()
    }
}

impl From<(&Plant, &str)> for PlantLink {
    fn from((plant, plant_base): (&Plant, &str)) -> PlantLink {
        PlantLink {
            plant_name: plant.name.clone(),
            plant_url: plant.get_url(plant_base),
        }
    }
}
