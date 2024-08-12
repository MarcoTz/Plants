pub mod species_list;
use super::{
    components::{html_head::HtmlHead, page_component::PageComponent},
    page::Page,
    shared::{footer::Footer, header::Header},
};
use species_list::SpeciesList;

use html::{body::Body, head::Head, html_document::HtmlDocument};
use std::rc::Rc;

pub struct SpeciesOverview {
    pub head: HtmlHead,
    pub header: Header,
    pub footer: Footer,
    pub species_list: SpeciesList,
}

impl Page for SpeciesOverview {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let body_contents = vec![
            self.header.render(date_format),
            self.species_list.render(date_format),
            self.footer.render(date_format),
        ];
        let body = Body {
            content: Rc::new(body_contents.into()),
        };
        HtmlDocument {
            head: Head::from(&self.head),
            body,
        }
    }
}
