use super::{
    super::html::{
        attribute::Attribute,
        body::Body,
        div::Div,
        head::Head,
        headline::{HeaderSize, Headline},
        html_document::HtmlDocument,
    },
    components::{
        footer::Footer, header::Header, species_gallery::SpeciesGallery, species_info::SpeciesInfo,
    },
    page::{Page, PageComponent},
};
use std::rc::Rc;
pub struct SpeciesDetails {
    species_name: String,
    species_info: SpeciesInfo,
    species_gallery: SpeciesGallery,
    header: Header,
    footer: Footer,
}

impl Page for SpeciesDetails {
    fn render(&self) -> HtmlDocument {
        let head = Head {
            title: self.species_name.clone(),
        };
        let body_contents = vec![
            self.header.render(),
            Headline {
                size: HeaderSize::H1,
                content: Rc::new(self.species_name.clone().into()),
            }
            .into(),
            Div {
                attributes: vec![Attribute::Id("species_content".to_owned())],
                content: Rc::new(self.species_info.render()),
            }
            .into(),
            self.species_gallery.render(),
            self.footer.render(),
        ];
        let body = Body {
            content: Rc::new(body_contents.into()),
        };
        HtmlDocument { head, body }
    }
}
