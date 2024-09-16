use crate::page::PageComponent;
use chrono::NaiveDate;
use html::{
    attribute::Attribute,
    elements::{HtmlElement, Table, Td, Tr},
};
use plants::plant::Plant;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
struct ActivityRow {
    date: NaiveDate,
    activity: String,
    plants: Vec<String>,
    notes: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActivitiesTable {
    activity_rows: Vec<ActivityRow>,
}

impl PageComponent for ActivitiesTable {
    fn render(&self, date_format: &str) -> HtmlElement {
        let header_row = Tr {
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
        };

        let mut table_rows = vec![header_row.into()];
        let mut activities_sorted = self.activity_rows.clone();
        activities_sorted.sort_by(|it1, it2| it2.date.cmp(&it1.date));
        for activity_row in activities_sorted.iter() {
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
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new(self.date.format(date_format).to_string().into()),
                },
                Td {
                    content: Rc::new(self.activity.clone().into()),
                },
                Td {
                    content: Rc::new(self.plants.join(", ").clone().into()),
                },
                Td {
                    content: Rc::new(self.notes.clone().into()),
                },
            ],
        }
        .into()
    }
}
impl From<&[Plant]> for ActivitiesTable {
    fn from(plants: &[Plant]) -> ActivitiesTable {
        log::info!("Generating activity table (all activities)");
        let mut activities: HashMap<(NaiveDate, String), (Vec<String>, String)> = HashMap::new();

        for plant in plants.iter() {
            for log in plant.activities.iter() {
                let log_key = (log.date, log.activity.clone());
                let note = log.note.clone().unwrap_or("".to_owned());

                match activities.get_mut(&log_key) {
                    None => {
                        activities.insert(log_key, (vec![plant.info.name.clone()], note));
                    }
                    Some((plants, notes)) => {
                        plants.push(plant.info.name.clone());
                        if !notes.contains(&note) {
                            notes.push_str(
                                &log.note
                                    .clone()
                                    .map(|note| ", ".to_owned() + &note)
                                    .unwrap_or("".to_owned()),
                            );
                        }
                    }
                }
            }
        }

        let mut activity_rows = vec![];

        for ((date, activity), (plants, notes)) in activities.iter() {
            activity_rows.push(ActivityRow {
                date: *date,
                activity: activity.clone(),
                plants: plants.clone(),
                notes: notes.clone(),
            });
        }
        activity_rows.sort_by(|row1, row2| row1.date.cmp(&row2.date));

        ActivitiesTable { activity_rows }
    }
}

#[cfg(test)]
mod activities_table_tests {

    use super::{ActivitiesTable, ActivityRow, PageComponent};
    use crate::test_common::{
        example_plant1, example_plant2, example_plant3, sample_date1, sample_date2, DATE_FORMAT,
    };
    use html::{
        attribute::Attribute,
        elements::{HtmlElement, Table, Td, Tr},
    };
    use std::rc::Rc;

    fn example_row1() -> ActivityRow {
        ActivityRow {
            date: sample_date1(),
            activity: "Watering".to_owned(),
            plants: vec![
                "Plant1".to_owned(),
                "Plant2".to_owned(),
                "Plant3".to_owned(),
            ],
            notes: "a note, a second note".to_owned(),
        }
    }

    fn example_row1_rendered() -> HtmlElement {
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
        .into()
    }

    fn example_row2() -> ActivityRow {
        ActivityRow {
            date: sample_date2(),
            activity: "Fertilizing".to_owned(),
            plants: vec!["Plant1".to_owned(), "Plant3".to_owned()],
            notes: "a different note".to_owned(),
        }
    }

    fn example_row2_rendered() -> HtmlElement {
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
        .into()
    }

    fn example_table() -> ActivitiesTable {
        ActivitiesTable {
            activity_rows: vec![example_row1(), example_row2()],
        }
    }

    fn example_table_rendered() -> HtmlElement {
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
                example_row2_rendered(),
                example_row1_rendered(),
            ],
        }
        .into()
    }

    #[test]
    fn render_table() {
        let result = example_table().render(DATE_FORMAT);
        let expected = example_table_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_row1() {
        let result = example_row1().render(DATE_FORMAT);
        let expected = example_row1_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_row2() {
        let result = example_row2().render(DATE_FORMAT);
        let expected = example_row2_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn table_from_plants() {
        let result = ActivitiesTable::from(
            vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
        );
        let expected = example_table();
        assert_eq!(result, expected)
    }
}
