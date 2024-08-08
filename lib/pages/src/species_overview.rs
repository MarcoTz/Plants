use super::{
    components::{
        footer::Footer, header::Header, html_head::HtmlHead, page_component::PageComponent,
        species_list::SpeciesList,
    },
    page::Page,
};

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
