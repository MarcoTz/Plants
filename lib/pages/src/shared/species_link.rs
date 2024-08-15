use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{HtmlElement, A},
};
use plants::species::Species;
use std::rc::Rc;

#[derive(Clone)]
pub struct SpeciesLink {
    pub species_name: String,
    pub species_url: String,
}

impl PageComponent for SpeciesLink {
    fn render(&self, _: &str) -> HtmlElement {
        A {
            attributes: vec![Attribute::Href(self.species_url.clone())],
            content: Rc::new(self.species_name.clone().into()),
        }
        .into()
    }
}

impl From<(&Species, &str)> for SpeciesLink {
    fn from((species, species_base): (&Species, &str)) -> SpeciesLink {
        SpeciesLink {
            species_name: species.name.clone(),
            species_url: species.get_url(species_base),
        }
    }
}
