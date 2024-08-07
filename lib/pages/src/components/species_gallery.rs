use super::{page_component::PageComponent, plant_gallery::PlantGallery};
use html::{attribute::Attribute, div::Div, html_element::HtmlElement};
use std::rc::Rc;

pub struct SpeciesGallery {
    plant_galleries: Vec<PlantGallery>,
}

impl PageComponent for SpeciesGallery {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut galleries = vec![];
        for plant_gallery in self.plant_galleries.iter() {
            galleries.push(plant_gallery.render(date_format));
        }
        Div {
            attributes: vec![Attribute::Id("species_details_gallery".to_owned())],
            content: Rc::new(galleries.into()),
        }
        .into()
    }
}
