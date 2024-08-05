use super::super::{
    super::html_components::{
        canvas::Canvas,
        component::HtmlComponent,
        div::Div,
        headline::{HeaderSize, Headline},
    },
    page::PageComponent,
};
use std::rc::Rc;

pub struct PlantGrowth {}

impl PageComponent for PlantGrowth {
    fn render(&self) -> HtmlComponent {
        let div_contents = vec![
            Headline {
                size: HeaderSize::H2,
                contents: Rc::new("Growth Log".to_owned().into()),
            }
            .into(),
            Canvas {
                id: "growth_chart".to_owned(),
            }
            .into(),
        ]
        .into();
        Div {
            id: Some("plant_growth_log_container".to_owned()),
            class: None,
            contents: Rc::new(div_contents),
        }
        .into()
    }
}
