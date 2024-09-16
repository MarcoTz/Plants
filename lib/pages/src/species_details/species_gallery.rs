use crate::{page::PageComponent, shared::plant_gallery::PlantGallery};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement},
};
use plants::plant::Plant;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
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
        let img_base = "../img/";
        let mut plant_galleries = vec![];
        for plant in plants.iter() {
            plant_galleries.push(PlantGallery::from((plant, img_base)))
        }
        SpeciesGallery { plant_galleries }
    }
}

#[cfg(test)]
mod species_gallery_tests {
    use super::{PageComponent, PlantGallery, SpeciesGallery};
    use crate::test_common::{example_plant1, example_plant2, example_plant3, DATE_FORMAT};
    use html::{
        attribute::Attribute,
        elements::{Div, HeaderSize, Headline},
    };
    use std::rc::Rc;

    fn example_gallery() -> SpeciesGallery {
        SpeciesGallery {
            plant_galleries: vec![
                PlantGallery::from((&example_plant1(), "../img")),
                PlantGallery::from((&example_plant2(), "../img")),
                PlantGallery::from((&example_plant3(), "../img")),
            ],
        }
    }

    #[test]
    fn render_gallery() {
        let result = example_gallery().render(DATE_FORMAT);
        let expected = Div {
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
                    vec![
                        PlantGallery::from((&example_plant1(), "../img")).render(DATE_FORMAT),
                        PlantGallery::from((&example_plant2(), "../img")).render(DATE_FORMAT),
                        PlantGallery::from((&example_plant3(), "../img")).render(DATE_FORMAT),
                    ]
                    .into(),
                ]
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn gallery_into() {
        let result = SpeciesGallery::from(
            vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
        );
        let expected = example_gallery();
        assert_eq!(result, expected)
    }
}
