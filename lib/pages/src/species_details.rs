use super::{
    components::{
        page_component::PageComponent, species_gallery::SpeciesGallery, species_info::SpeciesInfo,
    },
    page::Page,
    shared::{footer::Footer, header::Header, html_head::HtmlHead},
};
use html::{
    attribute::Attribute,
    body::Body,
    div::Div,
    head::Head,
    headline::{HeaderSize, Headline},
    html_document::HtmlDocument,
};
use plants::{plant::Plant, species::Species};
use std::rc::Rc;

pub struct SpeciesDetails {
    pub species_name: String,
    pub species_info: SpeciesInfo,
    pub species_gallery: SpeciesGallery,
    pub header: Header,
    pub footer: Footer,
}

impl Page for SpeciesDetails {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let body_contents = vec![
            self.header.render(date_format),
            Headline {
                attributes: vec![],
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
            head: Head::from(&self.get_head()),
            body,
        }
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "../css/main.css".to_owned(),
            "../css/header.css".to_owned(),
            "../css/footer.css".to_owned(),
        ];
        HtmlHead {
            title: self.species_name.clone(),
            styles,
        }
    }
}

impl From<(&Species, &[Plant])> for SpeciesDetails {
    fn from((species, plants): (&Species, &[Plant])) -> SpeciesDetails {
        let img_base = "img/plants";
        let species_plants = species.get_plants(plants);
        SpeciesDetails {
            header: Header::from(true),
            species_name: species.name.clone(),
            species_info: SpeciesInfo::from((species, species_plants.as_slice())),
            species_gallery: SpeciesGallery::from((species_plants.as_slice(), img_base)),
            footer: Footer::from(plants.len() as i32),
        }
    }
}
