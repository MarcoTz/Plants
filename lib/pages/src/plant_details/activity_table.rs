use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{HtmlElement, Table, Td, Tr},
};
use plants::log_item::LogItem;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct ActivityRow {
    activity: LogItem,
    include_activity: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActivityTable {
    activity_rows: Vec<ActivityRow>,
    include_activity: bool,
}

impl PageComponent for ActivityTable {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut header_row = Tr {
            attributes: vec![Attribute::Id("header_row".to_owned())],
            cols: vec![Td {
                content: Rc::new("Date".to_owned().into()),
            }],
        };

        if self.include_activity {
            header_row.cols.push(Td {
                content: Rc::new("Activity".to_owned().into()),
            });
        }

        header_row.cols.push(Td {
            content: Rc::new("Note".to_owned().into()),
        });

        let mut table_rows = vec![header_row.into()];
        for activity_row in self.activity_rows.iter() {
            table_rows.push(activity_row.render(date_format));
        }

        Table {
            attributes: vec![],
            rows: table_rows,
        }
        .into()
    }
}

impl PageComponent for ActivityRow {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut cols = vec![Td {
            content: Rc::new(self.activity.date.format(date_format).to_string().into()),
        }];
        if self.include_activity {
            cols.push(Td {
                content: Rc::new(self.activity.activity.clone().into()),
            });
        }

        cols.push(Td {
            content: Rc::new(self.activity.note.clone().unwrap_or("".to_owned()).into()),
        });
        Tr {
            attributes: vec![],
            cols,
        }
        .into()
    }
}

impl From<(&[&LogItem], bool)> for ActivityTable {
    fn from((logs, include_activity): (&[&LogItem], bool)) -> ActivityTable {
        log::info!("Loading Activity Table");
        ActivityTable {
            activity_rows: logs
                .iter()
                .cloned()
                .map(|x| (x, include_activity).into())
                .collect(),
            include_activity,
        }
    }
}
impl From<(&LogItem, bool)> for ActivityRow {
    fn from((log, include_activity): (&LogItem, bool)) -> ActivityRow {
        ActivityRow {
            activity: log.clone(),
            include_activity,
        }
    }
}

#[cfg(test)]
mod activity_table_tests {

    use super::{ActivityRow, ActivityTable, PageComponent};
    use crate::test_common::{example_activity1, sample_date1, DATE_FORMAT};
    use html::{
        attribute::Attribute,
        elements::{Table, Td, Tr},
    };
    use std::rc::Rc;

    fn example_row(include_activity: bool) -> ActivityRow {
        ActivityRow {
            activity: example_activity1("Plant1".to_owned(), "a note".to_owned()),
            include_activity,
        }
    }

    fn example_table(include_activity: bool) -> ActivityTable {
        ActivityTable {
            activity_rows: vec![example_row(include_activity)],
            include_activity,
        }
    }

    #[test]
    fn render_activity_table1() {
        let result = example_table(false).render(DATE_FORMAT);
        let expected = Table {
            attributes: vec![],
            rows: vec![
                Tr {
                    attributes: vec![Attribute::Id("header_row".to_owned())],
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
                            content: Rc::new(sample_date1().format(DATE_FORMAT).to_string().into()),
                        },
                        Td {
                            content: Rc::new("a note".to_owned().into()),
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
    fn render_activity_table2() {
        let result = example_table(true).render(DATE_FORMAT);
        let expected = Table {
            attributes: vec![],
            rows: vec![
                Tr {
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
                .into(),
                Tr {
                    attributes: vec![],
                    cols: vec![
                        Td {
                            content: Rc::new(sample_date1().format(DATE_FORMAT).to_string().into()),
                        },
                        Td {
                            content: Rc::new("Watering".to_owned().into()),
                        },
                        Td {
                            content: Rc::new("a note".to_owned().into()),
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
    fn render_activity_row1() {
        let result = example_row(false).render(DATE_FORMAT);
        let expected = Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new(sample_date1().format(DATE_FORMAT).to_string().into()),
                },
                Td {
                    content: Rc::new("a note".to_owned().into()),
                },
            ],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_activity_row2() {
        let result = example_row(true).render(DATE_FORMAT);
        let expected = Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new(sample_date1().format(DATE_FORMAT).to_string().into()),
                },
                Td {
                    content: Rc::new("Watering".to_owned().into()),
                },
                Td {
                    content: Rc::new("a note".to_owned().into()),
                },
            ],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn table_into1() {
        let result = ActivityTable::from((
            vec![&example_activity1("Plant1".to_owned(), "a note".to_owned())].as_slice(),
            false,
        ));
        let expected = example_table(false);
        assert_eq!(result, expected)
    }

    #[test]
    fn table_into2() {
        let result = ActivityTable::from((
            vec![&example_activity1("Plant1".to_owned(), "a note".to_owned())].as_slice(),
            true,
        ));
        let expected = ActivityTable {
            activity_rows: vec![example_row(true)],
            include_activity: true,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn row_into1() {
        let result = ActivityRow::from((
            &example_activity1("Plant1".to_owned(), "a note".to_owned()),
            false,
        ));
        let expected = example_row(false);
        assert_eq!(result, expected)
    }

    #[test]
    fn row_into2() {
        let result = ActivityRow::from((
            &example_activity1("Plant1".to_owned(), "a note".to_owned()),
            true,
        ));
        let expected = example_row(true);
        assert_eq!(result, expected)
    }
}
