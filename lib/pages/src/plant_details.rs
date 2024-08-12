use super::{
    components::{
        html_head::HtmlHead, page_component::PageComponent, plant_contents::PlantContents,
    },
    page::Page,
    shared::{footer::Footer, header::Header},
};
use html::{
    body::Body,
    head::Head,
    headline::{HeaderSize, Headline},
    html_document::HtmlDocument,
};
use std::rc::Rc;

pub struct PlantDetails {
    pub head: HtmlHead,
    pub plant_name: String,
    pub plant_species: Option<String>,
    pub header: Header,
    pub footer: Footer,
    pub plant_content: PlantContents,
}

impl Page for PlantDetails {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let plant_header = Headline {
            attributes: vec![],
            size: HeaderSize::H1,
            content: {
                let plant_name_str = self.plant_name.clone();
                let plant_species_str = self.plant_species.clone().unwrap_or("".to_owned());
                Rc::new(format!("{plant_name_str} {plant_species_str}").into())
            },
        }
        .into();
        let body_content = vec![
            self.header.render(date_format),
            plant_header,
            self.plant_content.render(date_format),
            self.footer.render(date_format),
        ]
        .into();
        let body = Body {
            content: Rc::new(body_content),
        };
        HtmlDocument {
            head: Head::from(&self.head),
            body,
        }
    }
}
