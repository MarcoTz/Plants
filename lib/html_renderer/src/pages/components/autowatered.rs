use super::super::{
    super::html_components::{
        component::HtmlComponent,
        div::Div,
        headline::{HeaderSize, Headline},
    },
    page::PageComponent,
};
use std::rc::Rc;

pub struct AutoWatered {
    auto_watered_plants: Vec<String>,
}

impl PageComponent for AutoWatered {
    fn render(&self) -> HtmlComponent {
        let auto_water_header = Headline {
            size: HeaderSize::H1,
            contents: Rc::new("Autowatered Plants".to_owned().into()),
        }
        .into();
        let mut plant_items = vec![];
        for auto_water_plant in self.auto_watered_plants.iter() {
            let new_div: HtmlComponent = Div {
                class: Some("auto_watered_plants".to_owned()),
                id: None,
                contents: Rc::new(auto_water_plant.clone().into()),
            }
            .into();
            plant_items.push(new_div);
        }
        vec![
            auto_water_header,
            Div {
                class: None,
                id: Some("autowatering_container".to_owned()),
                contents: Rc::new(plant_items.into()),
            }
            .into(),
        ]
        .into()
    }
}
