use crate::page::PageComponent;
use chrono::NaiveDate;
use html::{
    attribute::Attribute,
    elements::{HtmlElement, Table, Td, Tr},
};
use plants::plant::Plant;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
struct ActivityRow {
    date: NaiveDate,
    activity: String,
    plants: Vec<String>,
    notes: String,
}
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

        ActivitiesTable { activity_rows }
    }
}
