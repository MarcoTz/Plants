use super::{
    components::{
        footer::Footer, graveyard_table::GraveyardTable, header::Header,
        page_component::PageComponent,
    },
    page::Page,
};
use html::{body::Body, head::Head, html_document::HtmlDocument};
use std::rc::Rc;

pub struct Graveyard {
    header: Header,
    footer: Footer,
    graveyard_table: GraveyardTable,
}

impl Page for Graveyard {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let head = Head {
            title: "Graveyard".to_owned(),
        };
        let body_contents = vec![
            self.header.render(date_format),
            self.graveyard_table.render(date_format),
            self.footer.render(date_format),
        ];
        let body = Body {
            content: Rc::new(body_contents.into()),
        };
        HtmlDocument { head, body }
    }
}
