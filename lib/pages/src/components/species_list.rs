use super::page_component::PageComponent;
use html::{attribute::Attribute, div::Div, html_element::HtmlElement, img::Img, link::Link};
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
    fn render(&self) -> HtmlElement {
        let mut items = vec![];
        for species_item in self.species_items.iter() {
            items.push(species_item.render());
        }
        Div {
            attributes: vec![Attribute::Id("plant_list".to_owned())],
            content: Rc::new(items.into()),
        }
        .into()
    }
}

impl PageComponent for SpeciesListItem {
    fn render(&self) -> HtmlElement {
        let species_img: HtmlElement = match self.species_preview_url.clone() {
            None => "".to_owned().into(),
            Some(url) => Img {
                attributes: vec![Attribute::Href(url)],
            }
            .into(),
        };
        Div {
            attributes: vec![Attribute::Id("species_list_item".to_owned())],
            content: Rc::new(
                vec![
                    Link {
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
