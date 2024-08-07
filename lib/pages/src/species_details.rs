use super::{
    components::{
        footer::Footer, header::Header, html_head::HtmlHead, page_component::PageComponent,
        species_gallery::SpeciesGallery, species_info::SpeciesInfo,
    },
    page::Page,
};
use html::{
    attribute::Attribute,
    body::Body,
    div::Div,
    head::Head,
    headline::{HeaderSize, Headline},
    html_document::HtmlDocument,
};
use std::rc::Rc;

pub struct SpeciesDetails {
    head: HtmlHead,
    species_name: String,
    species_info: SpeciesInfo,
    species_gallery: SpeciesGallery,
    header: Header,
    footer: Footer,
}

impl Page for SpeciesDetails {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let body_contents = vec![
            self.header.render(date_format),
            Headline {
                size: HeaderSize::H1,
                content: Rc::new(self.species_name.clone().into()),
            }
            .into(),
            Div {
                attributes: vec![Attribute::Id("species_content".to_owned())],
                content: Rc::new(self.species_info.render(date_format)),
            }
            .into(),
            self.species_gallery.render(date_format),
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
