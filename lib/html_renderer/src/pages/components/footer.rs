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
        let mut footer_contents: Vec<HtmlComponent> = vec![];
        let github_link = Div {
            id: Some("github_link".to_owned()),
            class: None,
            contents: Rc::new(
                Link {
                    href: "https://github.com/MarcoTz/Plants".to_owned(),
                    contents: Rc::new("Github".to_owned().into()),
                }
                .into(),
            ),
        };
        footer_contents.push(github_link.into());

        let num_plants_str = self.num_plants.to_string();
        let num_plants: Div = Div {
            id: Some("num_plants".to_owned()),
            class: None,
            contents: Rc::new(format!("Number of plants: {num_plants_str}").into()),
        };
        footer_contents.push(num_plants.into());

        let last_build_str = self.last_build.format(&self.date_format);
        let last_build: Div = Div {
            id: Some("last_build".to_owned()),
            class: None,
            contents: Rc::new(format!("Last build: {last_build_str}").into()),
        };
        footer_contents.push(last_build.into());

        Div {
            id: Some("footer".to_owned()),
            class: None,
            contents: Rc::new(footer_contents.into()),
        }
        .into()
    }
}
