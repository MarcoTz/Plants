use crate::{page::PageComponent, shared::plant_link::PlantLink};
use html::{
    attribute::Attribute,
    div::Div,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
};

use plants::plant::Plant;
use std::rc::Rc;

pub struct AutoWatered {
    auto_watered_plants: Vec<PlantLink>,
}

impl PageComponent for AutoWatered {
    fn render(&self, date_format: &str) -> HtmlElement {
        let auto_water_header = Headline {
            attributes: vec![],
            size: HeaderSize::H1,
            content: Rc::new("Autowatered Plants".to_owned().into()),
        }
        .into();

        let mut plant_items = vec![];
        for auto_water_plant in self.auto_watered_plants.iter() {
            let plant_link = auto_water_plant.render(date_format);
            let new_div: HtmlElement = Div {
                attributes: vec![Attribute::Class(vec!["autowater_item".to_owned()])],
                content: Rc::new(plant_link),
            }
            .into();
            plant_items.push(new_div);
        }

        vec![
            auto_water_header,
            Div {
                attributes: vec![
                    Attribute::Id("autowatering_container".to_owned()),
                    Attribute::Class(vec![
                        "flex_container".to_owned(),
                        "alternating_children".to_owned(),
                    ]),
                ],
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
                plant_vec.push((plant, "plants").into())
            }
        }
        AutoWatered {
            auto_watered_plants: plant_vec,
        }
    }
}
