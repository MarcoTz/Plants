use super::{activity_table::ActivityTable, growth_table::GrowthTable};
use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement},
};
use plants::{growth_item::GrowthItem, log_item::LogItem, plant::Plant};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct PlantActivities {
    watering_table: ActivityTable,
    fertilizing_table: ActivityTable,
    activity_table: ActivityTable,
    growth_table: GrowthTable,
}

impl PageComponent for PlantActivities {
    fn render(&self, date_format: &str) -> HtmlElement {
        Div {
            attributes: vec![
                Attribute::Id("plant_activities_container".to_owned()),
                Attribute::Class(vec!["flex_container".to_owned()]),
            ],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    attributes: vec![],
                                    size: HeaderSize::H2,
                                    content: Rc::new("Watering".to_owned().into()),
                                }
                                .into(),
                                self.watering_table.render(date_format),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    attributes: vec![],
                                    size: HeaderSize::H2,
                                    content: Rc::new("Fertilizing".to_owned().into()),
                                }
                                .into(),
                                self.fertilizing_table.render(date_format),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    attributes: vec![],
                                    size: HeaderSize::H2,
                                    content: Rc::new("Other Activities".to_owned().into()),
                                }
                                .into(),
                                self.activity_table.render(date_format),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    attributes: vec![],
                                    size: HeaderSize::H2,
                                    content: Rc::new("Growth".to_owned().into()),
                                }
                                .into(),
                                self.growth_table.render(date_format),
                            ]
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

impl From<&Plant> for PlantActivities {
    fn from(plant: &Plant) -> PlantActivities {
        PlantActivities::from((plant.activities.as_slice(), plant.growth.as_slice()))
    }
}
impl From<(&[LogItem], &[GrowthItem])> for PlantActivities {
    fn from((logs, growth): (&[LogItem], &[GrowthItem])) -> PlantActivities {
        log::info!("Loading Plant Activities");
        let watering_activities: Vec<&LogItem> = logs
            .iter()
            .filter(|log| log.activity.to_lowercase().trim() == "watering")
            .collect();
        let fertilizing_activities: Vec<&LogItem> = logs
            .iter()
            .filter(|log| log.activity.to_lowercase().trim() == "fertilizing")
            .collect();
        let other_activities: Vec<&LogItem> = logs
            .iter()
            .filter(|x| !(watering_activities.contains(x) || fertilizing_activities.contains(x)))
            .collect();
        PlantActivities {
            watering_table: (watering_activities.as_slice(), false).into(),
            fertilizing_table: (fertilizing_activities.as_slice(), false).into(),
            activity_table: (other_activities.as_slice(), true).into(),
            growth_table: growth.into(),
        }
    }
}

#[cfg(test)]
mod details_activities_tests {
    use super::{ActivityTable, GrowthTable, PageComponent, PlantActivities};
    use crate::test_common::{
        example_plant1, example_plant2, example_plant3, sample_date1, sample_date2, DATE_FORMAT,
    };
    use html::{
        attribute::Attribute,
        elements::{Div, HeaderSize, Headline, Table, Td, Tr},
    };
    use plants::{growth_item::GrowthItem, log_item::LogItem};
    use std::rc::Rc;

    #[test]
    fn render_activities() {
        let result = PlantActivities::from(&example_plant3()).render(DATE_FORMAT);
        let expected = Div {
            attributes: vec![
                Attribute::Id("plant_activities_container".to_owned()),
                Attribute::Class(vec!["flex_container".to_owned()]),
            ],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    size: HeaderSize::H2,
                                    attributes: vec![],
                                    content: Rc::new("Watering".to_owned().into()),
                                }
                                .into(),
                                Table {
                                    attributes: vec![],
                                    rows: vec![
                                        Tr {
                                            attributes: vec![Attribute::Id(
                                                "header_row".to_owned(),
                                            )],
                                            cols: vec![
                                                Td {
                                                    content: Rc::new("Date".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("Note".to_owned().into()),
                                                },
                                            ],
                                        }
                                        .into(),
                                        Tr {
                                            attributes: vec![],
                                            cols: vec![
                                                Td {
                                                    content: Rc::new(
                                                        "01.01.1970".to_owned().into(),
                                                    ),
                                                },
                                                Td {
                                                    content: Rc::new("a note".to_owned().into()),
                                                },
                                            ],
                                        }
                                        .into(),
                                    ],
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    size: HeaderSize::H2,
                                    attributes: vec![],
                                    content: Rc::new("Fertilizing".to_owned().into()),
                                }
                                .into(),
                                Table {
                                    attributes: vec![],
                                    rows: vec![
                                        Tr {
                                            attributes: vec![Attribute::Id(
                                                "header_row".to_owned(),
                                            )],
                                            cols: vec![
                                                Td {
                                                    content: Rc::new("Date".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("Note".to_owned().into()),
                                                },
                                            ],
                                        }
                                        .into(),
                                        Tr {
                                            attributes: vec![],
                                            cols: vec![
                                                Td {
                                                    content: Rc::new(
                                                        "02.01.1970".to_owned().into(),
                                                    ),
                                                },
                                                Td {
                                                    content: Rc::new(
                                                        "a different note".to_owned().into(),
                                                    ),
                                                },
                                            ],
                                        }
                                        .into(),
                                    ],
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    size: HeaderSize::H2,
                                    attributes: vec![],
                                    content: Rc::new("Other Activities".to_owned().into()),
                                }
                                .into(),
                                Table {
                                    attributes: vec![],
                                    rows: vec![Tr {
                                        attributes: vec![Attribute::Id("header_row".to_owned())],
                                        cols: vec![
                                            Td {
                                                content: Rc::new("Date".to_owned().into()),
                                            },
                                            Td {
                                                content: Rc::new("Activity".to_owned().into()),
                                            },
                                            Td {
                                                content: Rc::new("Note".to_owned().into()),
                                            },
                                        ],
                                    }
                                    .into()],
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    size: HeaderSize::H2,
                                    attributes: vec![],
                                    content: Rc::new("Growth".to_owned().into()),
                                }
                                .into(),
                                Table {
                                    attributes: vec![],
                                    rows: vec![
                                        Tr {
                                            attributes: vec![Attribute::Id(
                                                "header_row".to_owned(),
                                            )],
                                            cols: vec![
                                                Td {
                                                    content: Rc::new("Date".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("Height".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("Width".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("Health".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("Note".to_owned().into()),
                                                },
                                            ],
                                        }
                                        .into(),
                                        Tr {
                                            attributes: vec![],
                                            cols: vec![
                                                Td {
                                                    content: Rc::new(
                                                        "01.01.1970".to_owned().into(),
                                                    ),
                                                },
                                                Td {
                                                    content: Rc::new("2".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("7.3".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("3".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("".to_owned().into()),
                                                },
                                            ],
                                        }
                                        .into(),
                                        Tr {
                                            attributes: vec![],
                                            cols: vec![
                                                Td {
                                                    content: Rc::new(
                                                        "02.01.1970".to_owned().into(),
                                                    ),
                                                },
                                                Td {
                                                    content: Rc::new("34.2".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("83.4".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("3".to_owned().into()),
                                                },
                                                Td {
                                                    content: Rc::new("".to_owned().into()),
                                                },
                                            ],
                                        }
                                        .into(),
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

    #[test]
    fn activities_from_plants() {
        let result = PlantActivities::from(&example_plant1());
        let expected = PlantActivities {
            watering_table: ActivityTable::from((
                vec![&LogItem {
                    activity: "Watering".to_owned(),
                    date: sample_date1(),
                    plant: "Plant1".to_owned(),
                    note: Some("a note".to_owned()),
                }]
                .as_slice(),
                false,
            )),
            fertilizing_table: ActivityTable::from((
                vec![&LogItem {
                    activity: "Fertilizing".to_owned(),
                    date: sample_date2(),
                    plant: "Plant1".to_owned(),
                    note: Some("a different note".to_owned()),
                }]
                .as_slice(),
                false,
            )),

            activity_table: ActivityTable::from((vec![].as_slice(), true)),
            growth_table: GrowthTable::from(
                vec![
                    GrowthItem {
                        plant: "Plant1".to_owned(),
                        date: sample_date1(),
                        height_cm: 50.0,
                        width_cm: 25.0,
                        note: None,
                        health: 4,
                    },
                    GrowthItem {
                        plant: "Plant1".to_owned(),
                        date: sample_date2(),
                        height_cm: 100.0,
                        width_cm: 75.0,
                        note: None,
                        health: 3,
                    },
                ]
                .as_slice(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn activities_from_log_growth() {
        let plant = example_plant2();
        let result = PlantActivities::from((plant.activities.as_slice(), plant.growth.as_slice()));
        let expected = PlantActivities {
            watering_table: ActivityTable::from((
                vec![&LogItem {
                    activity: "Watering".to_owned(),
                    date: sample_date1(),
                    plant: "Plant2".to_owned(),
                    note: Some("a second note".to_owned()),
                }]
                .as_slice(),
                false,
            )),
            fertilizing_table: ActivityTable::from((vec![].as_slice(), false)),
            activity_table: ActivityTable::from((vec![].as_slice(), true)),
            growth_table: GrowthTable::from(
                vec![
                    GrowthItem {
                        plant: "Plant2".to_owned(),
                        date: sample_date1(),
                        height_cm: 24.0,
                        width_cm: 18.3,
                        note: None,
                        health: 3,
                    },
                    GrowthItem {
                        plant: "Plant2".to_owned(),
                        date: sample_date2(),
                        height_cm: 75.3,
                        width_cm: 98.5,
                        note: None,
                        health: 3,
                    },
                ]
                .as_slice(),
            ),
        };
        assert_eq!(result, expected)
    }
}
