use super::super::{
    super::html::{
        attribute::Attribute,
        div::Div,
        headline::{HeaderSize, Headline},
        html_element::HtmlElement,
    },
    page::PageComponent,
};
use std::rc::Rc;

pub struct AutoWatered {
    auto_watered_plants: Vec<String>,
}

impl PageComponent for AutoWatered {
    fn render(&self) -> HtmlElement {
        let auto_water_header = Headline {
            size: HeaderSize::H1,
            content: Rc::new("Autowatered Plants".to_owned().into()),
        }
        .into();
        let mut plant_items = vec![];
        for auto_water_plant in self.auto_watered_plants.iter() {
            let new_div: HtmlElement = Div {
                attributes: vec![Attribute::Class("auto_watered_plants".to_owned())],
                content: Rc::new(auto_water_plant.clone().into()),
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
