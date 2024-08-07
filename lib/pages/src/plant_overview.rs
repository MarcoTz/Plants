use super::{
    components::{
        footer::Footer, header::Header, page_component::PageComponent, plant_list::PlantList,
        plant_search::PlantSearch,
    },
    page::Page,
};
use html::{body::Body, head::Head, html_document::HtmlDocument};
use std::rc::Rc;
pub struct PlantOverview {
    header: Header,
    search: PlantSearch,
    plant_list: PlantList,
    footer: Footer,
}

impl Page for PlantOverview {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let head = Head {
            title: "All Plants".to_owned(),
        };
        let body_content = vec![
            self.header.render(date_format),
            self.search.render(date_format),
            self.plant_list.render(date_format),
            self.footer.render(date_format),
        ];
        let body = Body {
            content: Rc::new(body_content.into()),
        };
        HtmlDocument { head, body }
    }
}
