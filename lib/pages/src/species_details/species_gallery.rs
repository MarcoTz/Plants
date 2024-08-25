use crate::{page::PageComponent, shared::plant_gallery::PlantGallery};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement},
};
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
            attributes: vec![
                Attribute::Id("species_details_gallery".to_owned()),
                Attribute::Class(vec![
                    "alternating_children".to_owned(),
                    "flex_container".to_owned(),
                ]),
            ],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H2,
                        content: Rc::new("Gallery".to_owned().into()),
                    }
                    .into(),
                    galleries.into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}

impl From<&[Plant]> for SpeciesGallery {
    fn from(plants: &[Plant]) -> SpeciesGallery {
        log::info!(
            "Loading species gallery with plants {}",
            plants
                .iter()
                .map(|plant| plant.info.name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        );
        let img_base = "../img/plants";
        let mut plant_galleries = vec![];
        for plant in plants.iter() {
            plant_galleries.push(PlantGallery::from((plant, img_base)))
        }
        SpeciesGallery { plant_galleries }
    }
}
