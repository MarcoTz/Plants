use super::{
    components::{
        footer::Footer, header::Header, page_component::PageComponent, plant_gallery::PlantGallery,
    },
    page::Page,
};
use html::{
    attribute::Attribute, body::Body, div::Div, head::Head, html_document::HtmlDocument,
    html_element::HtmlElement,
};
use std::rc::Rc;

pub struct Gallery {
    header: Header,
    footer: Footer,
    plant_galleries: Vec<PlantGallery>,
}

impl Page for Gallery {
    fn render(&self) -> HtmlDocument {
        let head = Head {
            title: "Gallery".to_owned(),
        };

        let galleries_rendered: Vec<HtmlElement> =
            self.plant_galleries.iter().map(|x| x.render()).collect();
        let body_content = vec![
            self.header.render(),
            Div {
                attributes: vec![Attribute::Id("plant_gallery".to_owned())],
                content: Rc::new(galleries_rendered.into()),
            }
            .into(),
            self.footer.render(),
        ];
        let body = Body {
            content: Rc::new(body_content.into()),
        };
        HtmlDocument { head, body }
    }
}
