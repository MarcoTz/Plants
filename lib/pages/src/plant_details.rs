use super::{
    components::{
        footer::Footer, header::Header, html_head::HtmlHead, page_component::PageComponent,
        plant_contents::PlantContents,
    },
    page::Page,
};
use html::{
    body::Body,
    head::Head,
    headline::{HeaderSize, Headline},
    html_document::HtmlDocument,
};
use std::rc::Rc;

pub struct PlantDetails {
    head: HtmlHead,
    plant_name: String,
    plant_species: String,
    header: Header,
    footer: Footer,
    plant_content: PlantContents,
}

impl Page for PlantDetails {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let plant_header = Headline {
            size: HeaderSize::H1,
            content: {
                let plant_name_str = self.plant_name.clone();
                let plant_species_str = self.plant_species.clone();

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
