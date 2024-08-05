use super::super::{
    super::html::{attribute::Attribute, div::Div, html_element::HtmlElement, link::Link},
    page::PageComponent,
};
use std::rc::Rc;

pub struct Header {
    dashboard_url: String,
    plants_url: String,
    species_url: String,
    gallery_url: String,
    activities_url: String,
    graveyard_url: String,
}

impl PageComponent for Header {
    fn render(&self) -> HtmlElement {
        let links: Vec<HtmlElement> = vec![
            Link {
                attributes: vec![Attribute::Href(self.dashboard_url.clone())],
                content: Rc::new("Dashboard".to_owned().into()),
            }
            .into(),
            Link {
                attributes: vec![Attribute::Href(self.plants_url.clone())],
                content: Rc::new("Plants".to_owned().into()),
            }
            .into(),
            Link {
                attributes: vec![Attribute::Href(self.species_url.clone())],
                content: Rc::new("Species".to_owned().into()),
            }
            .into(),
            Link {
                attributes: vec![Attribute::Href(self.gallery_url.clone())],
                content: Rc::new("Gallery".to_owned().into()),
            }
            .into(),
            Link {
                attributes: vec![Attribute::Href(self.activities_url.clone())],
                content: Rc::new("Activities".to_owned().into()),
            }
            .into(),
            Link {
                attributes: vec![Attribute::Href(self.graveyard_url.clone())],
                content: Rc::new("Graveyard".to_owned().into()),
            }
            .into(),
        ];
        Div {
            attributes: vec![Attribute::Id("header".to_owned())],
            content: Rc::new(links.into()),
        }
        .into()
    }
}
