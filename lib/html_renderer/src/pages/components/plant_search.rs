use super::super::{
    super::html_components::{
        component::HtmlComponent,
        div::Div,
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
    fn render(&self) -> HtmlComponent {
        let search_components = vec![
            Div {
                class: Some("search_header".to_owned()),
                id: None,
                content: Rc::new("Filter".to_owned().into()),
            }
            .into(),
            self.name_search.render(),
            self.species_search.render(),
            self.temp_min_search.render(),
            self.temp_max_search.render(),
        ];
        Div {
            class: None,
            id: Some("plant_search".to_owned()),
            content: Rc::new(search_components.into()),
        }
        .into()
    }
}

impl PageComponent for PlantSearchField {
    fn render(&self) -> HtmlComponent {
        Div {
            class: None,
            id: None,
            content: Rc::new(
                vec![
                    Div {
                        id: None,
                        class: None,
                        content: Rc::new(self.name.clone().into()),
                    }
                    .into(),
                    Div {
                        id: None,
                        class: None,
                        content: Rc::new(
                            Input {
                                ty: None,
                                id: Some(self.id.clone()),
                                keyup: Some(self.keyup.clone()),
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
    fn render(&self) -> HtmlComponent {
        Div {
            id: None,
            class: None,
            content: Rc::new(
                vec![
                    Div {
                        id: None,
                        class: None,
                        content: Rc::new(self.name.clone().into()),
                    }
                    .into(),
                    Select {
                        id: Some(self.select_id.clone()),
                        on_change: Some(self.select_on_change.clone()),
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
                        id: Some(self.input_id.clone()),
                        keyup: Some(self.input_on_keyup.clone()),
                        ty: Some("number".to_owned()),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
