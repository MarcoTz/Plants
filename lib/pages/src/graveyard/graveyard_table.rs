use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{HtmlElement, Table, Td, Tr},
};
use plants::graveyard::GraveyardPlant;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct GraveyardTable {
    plants: Vec<GraveyardPlant>,
}

impl PageComponent for GraveyardTable {
    fn render(&self, date_format: &str) -> HtmlElement {
        log::info!("Loading Graveyard Table");
        let mut table_rows = vec![Tr {
            attributes: vec![Attribute::Class(vec!["header_row".to_owned()])],
            cols: vec![
                Td {
                    content: Rc::new("Name".to_owned().into()),
                },
                Td {
                    content: Rc::new("Species".to_owned().into()),
                },
                Td {
                    content: Rc::new("Planted Date".to_owned().into()),
                },
                Td {
                    content: Rc::new("Died Date".to_owned().into()),
                },
                Td {
                    content: Rc::new("Cause of Death".to_owned().into()),
                },
            ],
        }
        .into()];
        for plant in self.plants.iter() {
            let new_row = Tr {
                attributes: vec![],
                cols: vec![
                    Td {
                        content: Rc::new(plant.name.clone().into()),
                    },
                    Td {
                        content: Rc::new(plant.species.clone().into()),
                    },
                    Td {
                        content: Rc::new(plant.planted.format(date_format).to_string().into()),
                    },
                    Td {
                        content: Rc::new(plant.died.format(date_format).to_string().into()),
                    },
                    Td {
                        content: Rc::new(plant.reason.clone().into()),
                    },
                ],
            }
            .into();
            table_rows.push(new_row);
        }
        Table {
            attributes: vec![],
            rows: table_rows,
        }
        .into()
    }
}

impl From<&[GraveyardPlant]> for GraveyardTable {
    fn from(plants: &[GraveyardPlant]) -> GraveyardTable {
        let mut gr_plants = vec![];
        gr_plants.extend_from_slice(plants);
        GraveyardTable { plants: gr_plants }
    }
}

#[cfg(test)]
mod graveyard_table_tests {
    use super::{GraveyardTable, PageComponent};
    use crate::test_common::{
        example_graveyard_plant1, example_graveyard_plant2, sample_date1, sample_date2, DATE_FORMAT,
    };
    use html::{
        attribute::Attribute,
        elements::{Table, Td, Tr},
    };
    use std::rc::Rc;

    fn example_graveyard_table() -> GraveyardTable {
        GraveyardTable {
            plants: vec![example_graveyard_plant1(), example_graveyard_plant2()],
        }
    }

    #[test]
    fn render_table() {
        let result = example_graveyard_table().render(DATE_FORMAT);
        let expected = Table {
            attributes: vec![],
            rows: vec![
                Tr {
                    attributes: vec![Attribute::Class(vec!["header_row".to_owned()])],
                    cols: vec![
                        Td {
                            content: Rc::new("Name".to_owned().into()),
                        },
                        Td {
                            content: Rc::new("Species".to_owned().into()),
                        },
                        Td {
                            content: Rc::new("Planted Date".to_owned().into()),
                        },
                        Td {
                            content: Rc::new("Died Date".to_owned().into()),
                        },
                        Td {
                            content: Rc::new("Cause of Death".to_owned().into()),
                        },
                    ],
                }
                .into(),
                Tr {
                    attributes: vec![],
                    cols: vec![
                        Td {
                            content: Rc::new("Plant1".to_owned().into()),
                        },
                        Td {
                            content: Rc::new("test species".to_owned().into()),
                        },
                        Td {
                            content: Rc::new(sample_date1().format(DATE_FORMAT).to_string().into()),
                        },
                        Td {
                            content: Rc::new(sample_date2().format(DATE_FORMAT).to_string().into()),
                        },
                        Td {
                            content: Rc::new("testing".to_owned().into()),
                        },
                    ],
                }
                .into(),
                Tr {
                    attributes: vec![],
                    cols: vec![
                        Td {
                            content: Rc::new("Plant2".to_owned().into()),
                        },
                        Td {
                            content: Rc::new("test species".to_owned().into()),
                        },
                        Td {
                            content: Rc::new(sample_date1().format(DATE_FORMAT).to_string().into()),
                        },
                        Td {
                            content: Rc::new(sample_date2().format(DATE_FORMAT).to_string().into()),
                        },
                        Td {
                            content: Rc::new("testing".to_owned().into()),
                        },
                    ],
                }
                .into(),
            ],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn table_into() {
        let result = GraveyardTable::from(
            vec![example_graveyard_plant1(), example_graveyard_plant2()].as_slice(),
        );
        let expected = example_graveyard_table();
        assert_eq!(result, expected)
    }
}
