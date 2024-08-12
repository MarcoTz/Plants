use super::{
    components::{page_component::PageComponent, plant_gallery::PlantGallery},
    page::Page,
    shared::{footer::Footer, header::Header, html_head::HtmlHead},
};
use html::{
    attribute::Attribute, body::Body, div::Div, head::Head, html_document::HtmlDocument,
    html_element::HtmlElement,
};
use plants::plant::Plant;
use std::rc::Rc;

pub struct Gallery {
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
            head: Head::from(&self.get_head()),
            body,
        }
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "css/main.css".to_owned(),
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
        ];
        HtmlHead {
            title: "Gallery".to_owned(),
            styles,
        }
    }
}

impl From<&[Plant]> for Gallery {
    fn from(plants: &[Plant]) -> Gallery {
        let img_base = "img/plants";
        let plant_galleries = plants.iter().map(|x| (x, img_base).into()).collect();
        Gallery {
            header: Header::from(false),
            plant_galleries,
            footer: Footer::from(plants.len() as i32),
        }
    }
}
