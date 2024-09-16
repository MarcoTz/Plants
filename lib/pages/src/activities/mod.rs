pub mod activities_table;
use super::{
    css::PageCss,
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use activities_table::ActivitiesTable;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement},
};
use plants::plant::Plant;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Activities {
    pub activity_table: ActivitiesTable,
}

impl Page for Activities {
    fn get_title(&self) -> String {
        "Activities".to_owned()
    }

    fn get_content(&self, date_format: &str) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Style("width:95%;margin:auto;".to_owned())],
            content: Rc::new(self.activity_table.render(date_format)),
        }
        .into()
    }

    fn get_head(&self, date_format: &str) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: self.get_title(),
            styles: PageCss::Activities,
            scripts,
            date_format: date_format.to_owned(),
        }
    }
}

impl From<&[Plant]> for Activities {
    fn from(plants: &[Plant]) -> Activities {
        Activities {
            activity_table: ActivitiesTable::from(plants),
        }
    }
}

#[cfg(test)]
mod activities_test {
    use super::{Activities, ActivitiesTable, Page};
    use crate::{
        css::PageCss,
        shared::html_head::HtmlHead,
        test_common::{example_plant1, example_plant2, example_plant3, DATE_FORMAT},
    };
    use html::{
        attribute::Attribute,
        elements::{Div, Table, Td, Tr},
    };
    use std::rc::Rc;

    fn example_activities() -> Activities {
        Activities {
            activity_table: ActivitiesTable::from(
                vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
            ),
        }
    }

    #[test]
    fn activities_title() {
        let result = example_activities().get_title();
        let expected = "Activities".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn activities_content() {
        let result = example_activities().get_content(DATE_FORMAT);
        let expected = Div {
            attributes: vec![Attribute::Style("width:95%;margin:auto;".to_owned())],
            content: Rc::new(
                Table {
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
                                    content: Rc::new("Plants".to_owned().into()),
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
                                    content: Rc::new("02.01.1970".to_owned().into()),
                                },
                                Td {
                                    content: Rc::new("Fertilizing".to_owned().into()),
                                },
                                Td {
                                    content: Rc::new("Plant1, Plant3".to_owned().into()),
                                },
                                Td {
                                    content: Rc::new("a different note".to_owned().into()),
                                },
                            ],
                        }
                        .into(),
                        Tr {
                            attributes: vec![],
                            cols: vec![
                                Td {
                                    content: Rc::new("01.01.1970".to_owned().into()),
                                },
                                Td {
                                    content: Rc::new("Watering".to_owned().into()),
                                },
                                Td {
                                    content: Rc::new("Plant1, Plant2, Plant3".to_owned().into()),
                                },
                                Td {
                                    content: Rc::new("a note, a second note".to_owned().into()),
                                },
                            ],
                        }
                        .into(),
                    ],
                }
                .into(),
            ),
        }
        .into();

        assert_eq!(result, expected)
    }

    #[test]
    fn activities_head() {
        let result = example_activities().get_head(DATE_FORMAT);
        let expected = HtmlHead {
            title: "Activities".to_owned(),
            styles: PageCss::Activities,
            scripts: vec!["js/main.js".to_owned()],
            date_format: DATE_FORMAT.to_owned(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn activities_from_plants() {
        let result =
            Activities::from(vec![example_plant1(), example_plant2(), example_plant3()].as_slice());
        let expected = example_activities();
        assert_eq!(result, expected)
    }
}
