use super::page_component::PageComponent;
use html::{
    attribute::Attribute,
    canvas::Canvas,
    div::Div,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
};
use std::rc::Rc;

pub struct PlantGrowth {}

impl PageComponent for PlantGrowth {
    fn render(&self) -> HtmlElement {
        let div_content = vec![
            Headline {
                size: HeaderSize::H2,
                content: Rc::new("Growth Log".to_owned().into()),
            }
            .into(),
            Canvas {
                attributes: vec![Attribute::Id("growth_chart".to_owned())],
            }
            .into(),
        ]
        .into();
        Div {
            attributes: vec![Attribute::Id("plant_growth_log_container".to_owned())],
            content: Rc::new(div_content),
        }
        .into()
    }
}
