use super::{
    components::{html_head::HtmlHead, page_component::PageComponent, plant_gallery::PlantGallery},
    page::Page,
    shared::{footer::Footer, header::Header},
};
use html::{
    attribute::Attribute, body::Body, div::Div, head::Head, html_document::HtmlDocument,
    html_element::HtmlElement,
};
use std::rc::Rc;

pub struct Gallery {
    pub head: HtmlHead,
    pub header: Header,
    pub footer: Footer,
    pub plant_galleries: Vec<PlantGallery>,
}

impl Page for Gallery {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let galleries_rendered: Vec<HtmlElement> = self
            .plant_galleries
            .iter()
            .map(|x| x.render(date_format))
            .collect();
        let body_content = vec![
            self.header.render(date_format),
            Div {
                attributes: vec![Attribute::Id("plant_gallery".to_owned())],
                content: Rc::new(galleries_rendered.into()),
            }
            .into(),
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
