pub mod plant_list;
pub mod plant_search;

use super::{components::page_component::PageComponent, errors::Error, page::Page};
use crate::shared::{footer::Footer, header::Header, html_head::HtmlHead};
use html::{body::Body, head::Head, html_document::HtmlDocument};
use plant_list::PlantList;
use plant_search::PlantSearch;
use plants::plant::Plant;
use std::rc::Rc;

pub struct PlantOverview {
    pub header: Header,
    pub search: PlantSearch,
    pub plant_list: PlantList,
    pub footer: Footer,
}

impl Page for PlantOverview {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let body_content = vec![
            self.header.render(date_format),
            self.search.render(date_format),
            self.plant_list.render(date_format),
            self.footer.render(date_format),
        ];
        let body = Body {
            content: Rc::new(body_content.into()),
        };
        HtmlDocument {
            head: Head::from(&self.get_head()),
            body,
        }
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "css/main.css".to_owned(),
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
            "css/plant_list.css".to_owned(),
            "css/plant_search.css".to_owned(),
        ];
        HtmlHead {
            title: "Plants".to_owned(),
            styles,
        }
    }
}

impl TryFrom<&[Plant]> for PlantOverview {
    type Error = Error;
    fn try_from(plants: &[Plant]) -> Result<PlantOverview, Self::Error> {
        let plant_list = PlantList::try_from(plants)?;
        Ok(PlantOverview {
            header: Header::from(false),
            search: PlantSearch {},
            plant_list,
            footer: Footer::from(plants.len() as i32),
        })
    }
}
