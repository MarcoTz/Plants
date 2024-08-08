use super::{
    components::{
        footer::Footer, header::Header, html_head::HtmlHead, page_component::PageComponent,
        plant_activity_table::PlantActivityTable,
    },
    page::Page,
};

use html::{body::Body, head::Head, html_document::HtmlDocument};
use std::rc::Rc;

pub struct Activities {
    pub head: HtmlHead,
    pub header: Header,
    pub footer: Footer,
    pub activity_table: PlantActivityTable,
}

impl Page for Activities {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let body_contents = vec![
            self.header.render(date_format),
            self.activity_table.render(date_format),
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
