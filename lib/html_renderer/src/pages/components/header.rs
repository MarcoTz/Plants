use super::super::{
    super::html_components::{component::HtmlComponent, div::Div, link::Link},
    page::PageComponent,
};
use std::rc::Rc;

pub struct Header {
    dashboard_link: String,
    plants_link: String,
    species_link: String,
    gallery_link: String,
    activities_link: String,
    graveyard_link: String,
}

impl PageComponent for Header {
    fn render(&self) -> HtmlComponent {
        let links: Vec<HtmlComponent> = vec![
            Link {
                class: None,
                href: self.dashboard_link.clone(),
                content: Rc::new("Dashboard".to_owned().into()),
            }
            .into(),
            Link {
                class: None,
                href: self.plants_link.clone(),
                content: Rc::new("Plants".to_owned().into()),
            }
            .into(),
            Link {
                class: None,
                href: self.species_link.clone(),
                content: Rc::new("Species".to_owned().into()),
            }
            .into(),
            Link {
                class: None,
                href: self.gallery_link.clone(),
                content: Rc::new("Gallery".to_owned().into()),
            }
            .into(),
            Link {
                class: None,
                href: self.activities_link.clone(),
                content: Rc::new("Activities".to_owned().into()),
            }
            .into(),
            Link {
                class: None,
                href: self.graveyard_link.clone(),
                content: Rc::new("Graveyard".to_owned().into()),
            }
            .into(),
        ];
        Div {
            class: None,
            id: Some("header".to_owned()),
            content: Rc::new(links.into()),
        }
        .into()
    }
}
