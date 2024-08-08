use super::{
    components::{
        footer::Footer, graveyard_table::GraveyardTable, header::Header, html_head::HtmlHead,
        page_component::PageComponent,
    },
    page::Page,
};
use html::{body::Body, head::Head, html_document::HtmlDocument};
use std::rc::Rc;

pub struct Graveyard {
    pub head: HtmlHead,
    pub header: Header,
    pub footer: Footer,
    pub graveyard_table: GraveyardTable,
}

impl Page for Graveyard {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let body_contents = vec![
            self.header.render(date_format),
            self.graveyard_table.render(date_format),
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
