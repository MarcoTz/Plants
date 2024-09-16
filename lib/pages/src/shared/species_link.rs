use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{HtmlElement, A},
};
use plants::species::Species;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod species_link_tests {
    use super::{PageComponent, SpeciesLink};
    use crate::test_common::{example_species, DATE_FORMAT};
    use html::{attribute::Attribute, elements::A};
    use std::rc::Rc;

    fn example_link() -> SpeciesLink {
        SpeciesLink {
            species_name: "test species".to_owned(),
            species_url: "species/testspecies.html".to_owned(),
        }
    }

    #[test]
    fn render_link() {
        let result = example_link().render(DATE_FORMAT);
        let expected = A {
            attributes: vec![Attribute::Href("species/testspecies.html".to_owned())],
            content: Rc::new("test species".to_owned().into()),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn link_from() {
        let result = SpeciesLink::from((&example_species(), "species"));
        let expected = example_link();
        assert_eq!(result, expected)
    }
}
