use super::super::{
    super::html_components::{component::HtmlComponent, div::Div, link::Link},
    page::PageComponent,
};
use chrono::NaiveDate;
use std::rc::Rc;

pub struct Footer {
    num_plants: i32,
    last_build: NaiveDate,
    date_format: String,
}

impl PageComponent for Footer {
    fn render(&self) -> HtmlComponent {
        let mut footer_content: Vec<HtmlComponent> = vec![];
        let github_link = Div {
            id: Some("github_link".to_owned()),
            class: None,
            content: Rc::new(
                Link {
                    class: None,
                    href: "https://github.com/MarcoTz/Plants".to_owned(),
                    content: Rc::new("Github".to_owned().into()),
                }
                .into(),
            ),
        };
        footer_content.push(github_link.into());

        let num_plants_str = self.num_plants.to_string();
        let num_plants: Div = Div {
            id: Some("num_plants".to_owned()),
            class: None,
            content: Rc::new(format!("Number of plants: {num_plants_str}").into()),
        };
        footer_content.push(num_plants.into());

        let last_build_str = self.last_build.format(&self.date_format);
        let last_build: Div = Div {
            id: Some("last_build".to_owned()),
            class: None,
            content: Rc::new(format!("Last build: {last_build_str}").into()),
        };
        footer_content.push(last_build.into());

        Div {
            id: Some("footer".to_owned()),
            class: None,
            content: Rc::new(footer_content.into()),
        }
        .into()
    }
}
