use crate::{page::PageComponent, shared::plant_link::PlantLink};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement},
};

use log;
use plants::plant::Plant;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct AutoWatered {
    auto_watered_plants: Vec<PlantLink>,
}

impl PageComponent for AutoWatered {
    fn render(&self, date_format: &str) -> HtmlElement {
        log::info!("Loading Autowatered Plants");
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
        log::info!("Getting autowatered plants");
        let mut plant_vec = vec![];
        for plant in plants.iter() {
            if plant.info.auto_water {
                plant_vec.push((plant, "plants").into())
            }
        }
        AutoWatered {
            auto_watered_plants: plant_vec,
        }
    }
}

#[cfg(test)]
mod auto_watered_test {
    use super::{AutoWatered, PageComponent};
    use crate::test_common::{
        example_plant1, example_plant2, example_plant3, example_plantlink1, example_plantlink2,
        DATE_FORMAT,
    };
    use html::{
        attribute::Attribute,
        elements::{Div, HeaderSize, Headline},
    };
    use std::rc::Rc;

    fn example_auto_watered() -> AutoWatered {
        AutoWatered {
            auto_watered_plants: vec![example_plantlink1(), example_plantlink2()],
        }
    }

    #[test]
    fn render_auto_watered() {
        let result = example_auto_watered().render(DATE_FORMAT);
        let expected = vec![
            Headline {
                attributes: vec![],
                size: HeaderSize::H1,
                content: Rc::new("Autowatered Plants".to_owned().into()),
            }
            .into(),
            Div {
                attributes: vec![
                    Attribute::Id("autowatering_container".to_owned()),
                    Attribute::Class(vec![
                        "flex_container".to_owned(),
                        "alternating_children".to_owned(),
                    ]),
                ],
                content: Rc::new(
                    vec![
                        Div {
                            attributes: vec![Attribute::Class(vec!["autowater_item".to_owned()])],
                            content: Rc::new(example_plantlink1().render(DATE_FORMAT)),
                        }
                        .into(),
                        Div {
                            attributes: vec![Attribute::Class(vec!["autowater_item".to_owned()])],
                            content: Rc::new(example_plantlink2().render(DATE_FORMAT)),
                        }
                        .into(),
                    ]
                    .into(),
                ),
            }
            .into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn auto_watered_into() {
        let result = AutoWatered::from(
            vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
        );
        let expected = AutoWatered {
            auto_watered_plants: vec![example_plantlink2()],
        };
        assert_eq!(result, expected)
    }
}
