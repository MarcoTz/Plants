pub mod species_gallery;
pub mod species_info;
use species_gallery::SpeciesGallery;
use species_info::SpeciesInfo;

use super::{
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use html::{
    attribute::Attribute,
    div::Div,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
};
use plants::{plant::Plant, species::Species};
use std::rc::Rc;

pub struct SpeciesDetails {
    pub species_name: String,
    pub species_info: SpeciesInfo,
    pub species_gallery: SpeciesGallery,
}

impl Page for SpeciesDetails {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        vec![
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
        ]
        .into()
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "../css/main.css".to_owned(),
            "../css/header.css".to_owned(),
            "../css/footer.css".to_owned(),
            "../css/gallery.css".to_owned(),
            "../css/species_details.css".to_owned(),
        ];
        let scripts = vec!["../js/main.js".to_owned()];
        HtmlHead {
            title: self.species_name.clone(),
            styles,
            scripts,
        }
    }
}

impl From<(&Species, &[Plant])> for SpeciesDetails {
    fn from((species, plants): (&Species, &[Plant])) -> SpeciesDetails {
        let species_plants = species.get_plants(plants);
        SpeciesDetails {
            species_name: species.name.clone(),
            species_info: SpeciesInfo::from((species, species_plants.as_slice())),
            species_gallery: SpeciesGallery::from(species_plants.as_slice()),
        }
    }
}