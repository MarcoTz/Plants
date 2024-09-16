pub mod graveyard_table;

use super::{
    css::PageCss,
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use graveyard_table::GraveyardTable;
use html::elements::HtmlElement;
use plants::graveyard::GraveyardPlant;

#[derive(Debug, PartialEq, Eq)]
pub struct Graveyard {
    pub graveyard_table: GraveyardTable,
}

impl Page for Graveyard {
    fn get_title(&self) -> String {
        "Graveyard".to_owned()
    }

    fn get_content(&self, date_format: &str) -> HtmlElement {
        self.graveyard_table.render(date_format)
    }

    fn get_head(&self, date_format: &str) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: self.get_title(),
            styles: PageCss::Graveyard,
            scripts,
            date_format: date_format.to_owned(),
        }
    }
}

impl From<&[GraveyardPlant]> for Graveyard {
    fn from(graveyard: &[GraveyardPlant]) -> Graveyard {
        log::info!("Loading Graveyard");
        Graveyard {
            graveyard_table: GraveyardTable::from(graveyard),
        }
    }
}

#[cfg(test)]
mod graveyard_tests {
    use super::{Graveyard, GraveyardTable, HtmlHead, Page, PageCss};
    use crate::test_common::{
        example_graveyard_plant1, example_graveyard_plant2, sample_date1, sample_date2, DATE_FORMAT,
    };
    use html::{
        attribute::Attribute,
        elements::{Table, Td, Tr},
    };
    use std::rc::Rc;

    fn example_graveyard() -> Graveyard {
        Graveyard {
            graveyard_table: GraveyardTable::from(
                vec![example_graveyard_plant1(), example_graveyard_plant2()].as_slice(),
            ),
        }
    }

    #[test]
    fn graveyard_get_title() {
        let result = example_graveyard().get_title();
        let expected = "Graveyard".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn graveyard_get_content() {
        let result = example_graveyard().get_content(DATE_FORMAT);
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
    fn graveyard_get_head() {
        let expected = example_graveyard().get_head(DATE_FORMAT);
        let result = HtmlHead {
            title: "Graveyard".to_owned(),
            styles: PageCss::Graveyard,
            scripts: vec!["js/main.js".to_owned()],
            date_format: DATE_FORMAT.to_owned(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn graveyard_into() {
        let result = Graveyard::from(
            vec![example_graveyard_plant1(), example_graveyard_plant2()].as_slice(),
        );
        let expected = example_graveyard();
        assert_eq!(result, expected)
    }
}
