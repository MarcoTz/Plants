use super::page_component::PageComponent;
use html::{
    a::A,
    attribute::Attribute,
    div::Div,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
};

use plants::plant::Plant;
use std::rc::Rc;

struct PlantLink {
    plant_name: String,
    plant_url: String,
}
pub struct AutoWatered {
    auto_watered_plants: Vec<PlantLink>,
}

impl PageComponent for AutoWatered {
    fn render(&self, _: &str) -> HtmlElement {
        let auto_water_header = Headline {
            size: HeaderSize::H1,
            content: Rc::new("Autowatered Plants".to_owned().into()),
        }
        .into();
        let mut plant_items = vec![];
        for auto_water_plant in self.auto_watered_plants.iter() {
            let plant_link = A {
                attributes: vec![Attribute::Href(auto_water_plant.plant_url.clone())],
                content: Rc::new(auto_water_plant.plant_name.clone().into()),
            }
            .into();
            let new_div: HtmlElement = Div {
                attributes: vec![Attribute::Class("autowater_item".to_owned())],
                content: Rc::new(plant_link),
            }
            .into();
            plant_items.push(new_div);
        }
        vec![
            auto_water_header,
            Div {
                attributes: vec![Attribute::Id("autowatering_container".to_owned())],
                content: Rc::new(plant_items.into()),
            }
            .into(),
        ]
        .into()
    }
}

impl From<&[Plant]> for AutoWatered {
    fn from(plants: &[Plant]) -> AutoWatered {
        let mut plant_vec = vec![];
        for plant in plants.iter() {
            if plant.auto_water {
                plant_vec.push(PlantLink {
                    plant_name: plant.name.clone(),
                    plant_url: plant.get_url("plants/"),
                })
            }
        }
        AutoWatered {
            auto_watered_plants: plant_vec,
        }
    }
}
