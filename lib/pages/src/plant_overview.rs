use super::{
    components::{
        footer::Footer, header::Header, html_head::HtmlHead, page_component::PageComponent,
        plant_list::PlantList, plant_search::PlantSearch,
    },
    page::Page,
};
use html::{body::Body, head::Head, html_document::HtmlDocument};
use std::rc::Rc;
pub struct PlantOverview {
    pub head: HtmlHead,
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
            head: Head::from(&self.head),
            body,
        }
    }
}
