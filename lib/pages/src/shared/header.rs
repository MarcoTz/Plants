use crate::components::page_component::PageComponent;
use html::{a::A, attribute::Attribute, div::Div, html_element::HtmlElement};
use std::rc::Rc;

pub struct Header {
    pub dashboard_url: String,
    pub plants_url: String,
    pub species_url: String,
    pub gallery_url: String,
    pub activities_url: String,
    pub graveyard_url: String,
}

impl PageComponent for Header {
    fn render(&self, _: &str) -> HtmlElement {
        let links: Vec<HtmlElement> = vec![
            A {
                attributes: vec![Attribute::Href(self.dashboard_url.clone())],
                content: Rc::new("Dashboard".to_owned().into()),
            }
            .into(),
            A {
                attributes: vec![Attribute::Href(self.plants_url.clone())],
                content: Rc::new("Plants".to_owned().into()),
            }
            .into(),
            A {
                attributes: vec![Attribute::Href(self.species_url.clone())],
                content: Rc::new("Species".to_owned().into()),
            }
            .into(),
            A {
                attributes: vec![Attribute::Href(self.gallery_url.clone())],
                content: Rc::new("Gallery".to_owned().into()),
            }
            .into(),
            A {
                attributes: vec![Attribute::Href(self.activities_url.clone())],
                content: Rc::new("Activities".to_owned().into()),
            }
            .into(),
            A {
                attributes: vec![Attribute::Href(self.graveyard_url.clone())],
                content: Rc::new("Graveyard".to_owned().into()),
            }
            .into(),
        ];
        Div {
            attributes: vec![
                Attribute::Class(vec![
                    "alternating_children".to_owned(),
                    "flex_container".to_owned(),
                ]),
                Attribute::Id("header".to_owned()),
            ],
            content: Rc::new(links.into()),
        }
        .into()
    }
}
