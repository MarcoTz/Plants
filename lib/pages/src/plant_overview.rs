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
    fn render(&self) -> HtmlDocument {
        let head = Head {
            title: "All Plants".to_owned(),
        };
        let body_content = vec![
            self.header.render(),
            self.search.render(),
            self.plant_list.render(),
            self.footer.render(),
        ];
        let body = Body {
            content: Rc::new(body_content.into()),
        };
        HtmlDocument { head, body }
    }
}
