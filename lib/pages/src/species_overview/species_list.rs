use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, Img, A},
};
use plants::{plant::Plant, species::Species};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct SpeciesListItem {
    species_url: String,
    species_name: String,
    species_preview_url: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SpeciesList {
    species_items: Vec<SpeciesListItem>,
}

impl PageComponent for SpeciesList {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut items = vec![];
        for species_item in self.species_items.iter() {
            items.push(species_item.render(date_format));
        }
        Div {
            attributes: vec![
                Attribute::Id("plant_list".to_owned()),
                Attribute::Class(vec![
                    "flex_container".to_owned(),
                    "alternating_children".to_owned(),
                ]),
            ],
            content: Rc::new(items.into()),
        }
        .into()
    }
}

impl PageComponent for SpeciesListItem {
    fn render(&self, _: &str) -> HtmlElement {
        log::info!("Loading species list");
        let species_img: HtmlElement = match self.species_preview_url.clone() {
            None => "".to_owned().into(),
            Some(url) => vec![
                HtmlElement::Br,
                Img {
                    attributes: vec![
                        Attribute::Src(url),
                        Attribute::Class(vec!["plant_preview".to_owned()]),
                    ],
                }
                .into(),
            ]
            .into(),
        };
        Div {
            attributes: vec![Attribute::Class(vec!["plant_list_item".to_owned()])],
            content: Rc::new(
                vec![
                    A {
                        attributes: vec![Attribute::Href(self.species_url.clone())],
                        content: Rc::new(self.species_name.clone().into()),
                    }
                    .into(),
                    species_img,
                ]
                .into(),
            ),
        }
        .into()
    }
}

impl From<&(&Species, Option<Plant>)> for SpeciesListItem {
    fn from((species, m_plant): &(&Species, Option<Plant>)) -> SpeciesListItem {
        let species_preview_url = match m_plant.clone().map(|p| p.get_preview_image_url("img/")) {
            None => None,
            Some(None) => None,
            Some(Some(url)) => Some(url),
        };
        SpeciesListItem {
            species_url: species.get_url("species"),
            species_name: species.name.clone(),
            species_preview_url,
        }
    }
}
impl From<&[(&Species, Option<Plant>)]> for SpeciesList {
    fn from(species: &[(&Species, Option<Plant>)]) -> SpeciesList {
        SpeciesList {
            species_items: species.iter().map(|sp| sp.into()).collect(),
        }
    }
}

#[cfg(test)]
mod species_list_tests {

    use super::{PageComponent, SpeciesList, SpeciesListItem};
    use crate::test_common::{example_plant1, example_species, DATE_FORMAT};
    use html::{
        attribute::Attribute,
        elements::{Div, A},
    };
    use std::rc::Rc;

    fn example_list() -> SpeciesList {
        SpeciesList {
            species_items: vec![example_item()],
        }
    }

    fn example_item() -> SpeciesListItem {
        let species = example_species();
        SpeciesListItem {
            species_url: species.get_url("species"),
            species_name: "test species".to_owned(),
            species_preview_url: None,
        }
    }

    #[test]
    fn render_list() {
        let result = example_list().render(DATE_FORMAT);
        let expected = Div {
            attributes: vec![
                Attribute::Id("plant_list".to_owned()),
                Attribute::Class(vec![
                    "flex_container".to_owned(),
                    "alternating_children".to_owned(),
                ]),
            ],
            content: Rc::new(vec![example_item().render(DATE_FORMAT)].into()),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_item() {
        let result = example_item().render(DATE_FORMAT);
        let expected = Div {
            attributes: vec![Attribute::Class(vec!["plant_list_item".to_owned()])],
            content: Rc::new(
                vec![
                    A {
                        attributes: vec![Attribute::Href(
                            example_species().get_url("species").clone(),
                        )],
                        content: Rc::new("test species".to_owned().into()),
                    }
                    .into(),
                    "".to_owned().into(),
                ]
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn item_into() {
        let result =
            SpeciesList::from(vec![(&example_species(), Some(example_plant1()))].as_slice());
        let expected = example_list();
        assert_eq!(result, expected)
    }

    #[test]
    fn list_into() {
        let result = SpeciesListItem::from(&(&example_species(), Some(example_plant1())));
        let expected = example_item();
        assert_eq!(result, expected)
    }
}
