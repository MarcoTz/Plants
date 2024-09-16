use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, Input, Select, SelectOption},
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

#[derive(Debug, PartialEq, Eq)]
pub struct PlantSearch;

impl PageComponent for PlantSearch {
    fn render(&self, date_format: &str) -> HtmlElement {
        let name_search = PlantSearchField {
            name: "Name".to_owned(),
            id: "plant_search_name".to_owned(),
            keyup: "filter_plants()".to_owned(),
        };
        let species_search = PlantSearchField {
            name: "Species".to_owned(),
            id: "plant_search_species".to_owned(),
            keyup: "filter_plants()".to_owned(),
        };
        let temp_min_search = PlantNumberSearch {
            name: "Lowest Temperature".to_owned(),
            select_id: "min_temp_updown".to_owned(),
            select_on_change: "filter_plants()".to_owned(),
            input_id: "plant_search_min_temp".to_owned(),
            input_on_keyup: "filter_plants()".to_owned(),
        };
        let temp_max_search = PlantNumberSearch {
            name: "Highest Temperature".to_owned(),
            select_id: "max_temp_updown".to_owned(),
            select_on_change: "filter_plants()".to_owned(),
            input_id: "plant_search_max_temp".to_owned(),
            input_on_keyup: "filter_plants()".to_owned(),
        };
        let search_components = vec![
            Div {
                attributes: vec![Attribute::Class(vec!["search_header".to_owned()])],
                content: Rc::new("Filter".to_owned().into()),
            }
            .into(),
            name_search.render(date_format),
            species_search.render(date_format),
            temp_min_search.render(date_format),
            temp_max_search.render(date_format),
        ];
        Div {
            attributes: vec![
                Attribute::Id("plant_search".to_owned()),
                Attribute::Class(vec!["flex_container".to_owned()]),
            ],
            content: Rc::new(search_components.into()),
        }
        .into()
    }
}

impl PageComponent for PlantSearchField {
    fn render(&self, _: &str) -> HtmlElement {
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
    fn render(&self, _: &str) -> HtmlElement {
        log::info!("Generating plant search");
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

#[cfg(test)]
mod plant_search_tests {
    use super::{PageComponent, PlantSearch};
    use crate::test_common::DATE_FORMAT;
    use html::{
        attribute::Attribute,
        elements::{Div, Input, Select, SelectOption},
    };
    use std::rc::Rc;

    #[test]
    fn render_search() {
        let result = PlantSearch {}.render(DATE_FORMAT);
        let expected = Div {
            attributes: vec![
                Attribute::Id("plant_search".to_owned()),
                Attribute::Class(vec!["flex_container".to_owned()]),
            ],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Class(vec!["search_header".to_owned()])],
                        content: Rc::new("Filter".to_owned().into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![],
                        content: Rc::new(
                            vec![
                                Div {
                                    attributes: vec![],
                                    content: Rc::new("Name".to_owned().into()),
                                }
                                .into(),
                                Div {
                                    attributes: vec![],
                                    content: Rc::new(
                                        Input {
                                            attributes: vec![
                                                Attribute::Id("plant_search_name".to_owned()),
                                                Attribute::OnKeyUp("filter_plants()".to_owned()),
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
                    .into(),
                    Div {
                        attributes: vec![],
                        content: Rc::new(
                            vec![
                                Div {
                                    attributes: vec![],
                                    content: Rc::new("Species".to_owned().into()),
                                }
                                .into(),
                                Div {
                                    attributes: vec![],
                                    content: Rc::new(
                                        Input {
                                            attributes: vec![
                                                Attribute::Id("plant_search_species".to_owned()),
                                                Attribute::OnKeyUp("filter_plants()".to_owned()),
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
                    .into(),
                    Div {
                        attributes: vec![],
                        content: Rc::new(
                            vec![
                                Div {
                                    attributes: vec![],
                                    content: Rc::new("Lowest Temperature".to_owned().into()),
                                }
                                .into(),
                                Select {
                                    attributes: vec![
                                        Attribute::Id("min_temp_updown".to_owned()),
                                        Attribute::OnChange("filter_plants()".to_owned()),
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
                                        Attribute::Id("plant_search_min_temp".to_owned()),
                                        Attribute::OnKeyUp("filter_plants()".to_owned()),
                                        Attribute::Type("number".to_owned()),
                                    ],
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![],
                        content: Rc::new(
                            vec![
                                Div {
                                    attributes: vec![],
                                    content: Rc::new("Highest Temperature".to_owned().into()),
                                }
                                .into(),
                                Select {
                                    attributes: vec![
                                        Attribute::Id("max_temp_updown".to_owned()),
                                        Attribute::OnChange("filter_plants()".to_owned()),
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
                                        Attribute::Id("plant_search_max_temp".to_owned()),
                                        Attribute::OnKeyUp("filter_plants()".to_owned()),
                                        Attribute::Type("number".to_owned()),
                                    ],
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
