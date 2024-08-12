use super::{super::shared::plant_gallery::PlantGallery, page_component::PageComponent};
use html::{attribute::Attribute, div::Div, html_element::HtmlElement};
use plants::plant::Plant;
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

impl From<(&[Plant], &str)> for SpeciesGallery {
    fn from((plants, img_base): (&[Plant], &str)) -> SpeciesGallery {
        let mut plant_galleries = vec![];
        for plant in plants.iter() {
            plant_galleries.push(PlantGallery::from((plant, img_base)))
        }
        SpeciesGallery { plant_galleries }
    }
}
