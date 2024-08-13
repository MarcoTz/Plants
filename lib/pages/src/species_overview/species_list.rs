use crate::components::page_component::PageComponent;
use html::{a::A, attribute::Attribute, div::Div, html_element::HtmlElement, img::Img};
use plants::{plant::Plant, species::Species};
use std::rc::Rc;

struct SpeciesListItem {
    species_url: String,
    species_name: String,
    species_preview_url: Option<String>,
}
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
        let species_preview_url = match m_plant
            .clone()
            .map(|p| p.get_preview_image_url("img/plants"))
        {
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
