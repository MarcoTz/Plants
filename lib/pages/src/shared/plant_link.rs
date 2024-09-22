use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{HtmlElement, A},
};
use plants::plant::Plant;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlantLink {
    pub plant_name: String,
    pub plant_url: String,
}

impl PageComponent for PlantLink {
    fn render(&self, _: &str) -> HtmlElement {
        A {
            attributes: vec![
                Attribute::Href(self.plant_url.clone()),
                Attribute::Class(vec!["plant_link".to_owned()]),
            ],
            content: Rc::new(self.plant_name.clone().into()),
        }
        .into()
    }
}

impl From<(&Plant, &str)> for PlantLink {
    fn from((plant, plant_base): (&Plant, &str)) -> PlantLink {
        PlantLink {
            plant_name: plant.info.name.clone(),
            plant_url: plant.get_url(plant_base),
        }
    }
}

#[cfg(test)]
mod plant_link_tests {
    use super::{PageComponent, PlantLink};
    use crate::test_common::{example_plant1, DATE_FORMAT};
    use html::{attribute::Attribute, elements::A};
    use std::rc::Rc;

    fn example_link() -> PlantLink {
        PlantLink {
            plant_name: "Plant1".to_owned(),
            plant_url: "plants/Plant1.html".to_owned(),
        }
    }

    #[test]
    fn render_link() {
        let result = example_link().render(DATE_FORMAT);
        let expected = A {
            attributes: vec![
                Attribute::Href("plants/Plant1.html".to_owned()),
                Attribute::Class(vec!["plant_link".to_owned()]),
            ],
            content: Rc::new("Plant1".to_owned().into()),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn link_into() {
        let result = PlantLink::from((&example_plant1(), "plants"));
        let expected = example_link();
        assert_eq!(result, expected)
    }
}
