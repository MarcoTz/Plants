use super::page_component::PageComponent;
use html::{attribute::Attribute, div::Div, html_element::HtmlElement, link::Link};
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
    fn render(&self, _: &str) -> HtmlElement {
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

impl From<(String, String, String, String, String, String)> for Header {
    fn from(
        (dashboard_url, plants_url, species_url, gallery_url, activities_url, graveyard_url): (
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    ) -> Header {
        Header {
            dashboard_url,
            plants_url,
            species_url,
            gallery_url,
            activities_url,
            graveyard_url,
        }
    }
}
