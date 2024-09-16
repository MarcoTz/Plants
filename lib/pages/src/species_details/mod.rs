pub mod species_gallery;
pub mod species_info;
use species_gallery::SpeciesGallery;
use species_info::SpeciesInfo;

use super::{
    css::PageCss,
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement},
};
use plants::{plant::Plant, species::Species};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct SpeciesDetails {
    pub species_name: String,
    pub species_info: SpeciesInfo,
    pub species_gallery: SpeciesGallery,
}

impl Page for SpeciesDetails {
    fn get_title(&self) -> String {
        self.species_name.clone()
    }

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

    fn get_head(&self, date_format: &str) -> HtmlHead {
        let scripts = vec!["../js/main.js".to_owned()];
        HtmlHead {
            title: self.get_title(),
            styles: PageCss::SpeciesDetails,
            scripts,
            date_format: date_format.to_owned(),
        }
    }
}

impl From<(&Species, &[Plant])> for SpeciesDetails {
    fn from((species, plants): (&Species, &[Plant])) -> SpeciesDetails {
        log::info!("Loading species details for {}", species.name);
        let species_plants = species.get_plants(plants);
        SpeciesDetails {
            species_name: species.name.clone(),
            species_info: SpeciesInfo::from((species, species_plants.as_slice())),
            species_gallery: SpeciesGallery::from(species_plants.as_slice()),
        }
    }
}

#[cfg(test)]
mod species_details_tests {
    use super::{
        HtmlHead, Page, PageComponent, PageCss, SpeciesDetails, SpeciesGallery, SpeciesInfo,
    };
    use crate::test_common::{
        example_plant1, example_plant2, example_plant3, example_species, DATE_FORMAT,
    };
    use html::{
        attribute::Attribute,
        elements::{Div, HeaderSize, Headline},
    };
    use std::rc::Rc;

    fn example_details() -> SpeciesDetails {
        SpeciesDetails {
            species_name: "test species".to_owned(),
            species_info: SpeciesInfo::from((
                &example_species(),
                vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
            )),
            species_gallery: SpeciesGallery::from(
                vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
            ),
        }
    }

    #[test]
    fn details_title() {
        let result = example_details().get_title();
        let expected = "test species";
        assert_eq!(result, expected)
    }

    #[test]
    fn details_content() {
        let result = example_details().get_content(DATE_FORMAT);
        let expected = vec![
            Headline {
                attributes: vec![],
                size: HeaderSize::H1,
                content: Rc::new("test species".to_owned().into()),
            }
            .into(),
            Div {
                attributes: vec![Attribute::Id("species_content".to_owned())],
                content: Rc::new(
                    SpeciesInfo::from((
                        &example_species(),
                        vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
                    ))
                    .render(DATE_FORMAT),
                ),
            }
            .into(),
            SpeciesGallery::from(
                vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
            )
            .render(DATE_FORMAT),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn details_head() {
        let result = example_details().get_head(DATE_FORMAT);
        let expected = HtmlHead {
            title: "test species".to_owned(),
            styles: PageCss::SpeciesDetails,
            scripts: vec!["../js/main.js".to_owned()],
            date_format: DATE_FORMAT.to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn details_into() {
        let result = SpeciesDetails::from((
            &example_species(),
            vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
        ));
        let expected = example_details();
        assert_eq!(result, expected)
    }
}
