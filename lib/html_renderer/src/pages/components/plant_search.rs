use super::super::{
    super::html::{
        attribute::Attribute,
        div::Div,
        html_element::HtmlElement,
        input::Input,
        select::{Select, SelectOption},
    },
    page::PageComponent,
};
use std::rc::Rc;

struct PlantSearchField {
    name: String,
    id: String,
    keyup: String,
}

struct PlantNumberSearch {
    name: String,
    select_id: String,
    select_on_change: String,
    input_id: String,
    input_on_keyup: String,
}
pub struct PlantSearch {
    name_search: PlantSearchField,
    species_search: PlantSearchField,
    temp_min_search: PlantNumberSearch,
    temp_max_search: PlantNumberSearch,
}

impl PageComponent for PlantSearch {
    fn render(&self) -> HtmlElement {
        let search_components = vec![
            Div {
                attributes: vec![Attribute::Class("search_header".to_owned())],
                content: Rc::new("Filter".to_owned().into()),
            }
            .into(),
            self.name_search.render(),
            self.species_search.render(),
            self.temp_min_search.render(),
            self.temp_max_search.render(),
        ];
        Div {
            attributes: vec![Attribute::Id("plant_search".to_owned())],
            content: Rc::new(search_components.into()),
        }
        .into()
    }
}

impl PageComponent for PlantSearchField {
    fn render(&self) -> HtmlElement {
        Div {
            attributes: vec![],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![],
                        content: Rc::new(self.name.clone().into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![],
                        content: Rc::new(
                            Input {
                                attributes: vec![
                                    Attribute::Id(self.id.clone()),
                                    Attribute::OnKeyUp(self.keyup.clone()),
                                ],
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}

impl PageComponent for PlantNumberSearch {
    fn render(&self) -> HtmlElement {
        Div {
            attributes: vec![],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![],
                        content: Rc::new(self.name.clone().into()),
                    }
                    .into(),
                    Select {
                        attributes: vec![
                            Attribute::Id(self.select_id.clone()),
                            Attribute::OnChange(self.select_on_change.clone()),
                        ],
                        options: vec![
                            SelectOption {
                                value: "+".to_owned(),
                                content: Rc::new("above".to_owned().into()),
                            },
                            SelectOption {
                                value: "-".to_owned(),
                                content: Rc::new("below".to_owned().into()),
                            },
                        ],
                    }
                    .into(),
                    Input {
                        attributes: vec![
                            Attribute::Id(self.input_id.clone()),
                            Attribute::OnKeyUp(self.input_on_keyup.clone()),
                            Attribute::Type("number".to_owned()),
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
