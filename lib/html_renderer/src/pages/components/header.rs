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
                href: self.dashboard_link.clone(),
                contents: Rc::new("Dashboard".to_owned().into()),
            }
            .into(),
            Link {
                href: self.plants_link.clone(),
                contents: Rc::new("Plants".to_owned().into()),
            }
            .into(),
            Link {
                href: self.species_link.clone(),
                contents: Rc::new("Species".to_owned().into()),
            }
            .into(),
            Link {
                href: self.gallery_link.clone(),
                contents: Rc::new("Gallery".to_owned().into()),
            }
            .into(),
            Link {
                href: self.activities_link.clone(),
                contents: Rc::new("Activities".to_owned().into()),
            }
            .into(),
            Link {
                href: self.graveyard_link.clone(),
                contents: Rc::new("Graveyard".to_owned().into()),
            }
            .into(),
        ];
        Div {
            class: None,
            id: Some("header".to_owned()),
            contents: Rc::new(links.into()),
        }
        .into()
    }
}
