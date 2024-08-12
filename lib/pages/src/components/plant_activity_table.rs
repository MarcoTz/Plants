use crate::components::page_component::PageComponent;
use html::{
    attribute::Attribute,
    html_element::HtmlElement,
    table::{Table, Td, Tr},
};
use plants::{log_item::LogItem, plant::Plant};
use std::rc::Rc;

struct PlantActivityRow {
    activity: LogItem,
    include_activity: bool,
    include_plant: bool,
}
pub struct PlantActivityTable {
    activity_rows: Vec<PlantActivityRow>,
    include_activity: bool,
    include_plant: bool,
}

impl PageComponent for PlantActivityTable {
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

        if self.include_plant {
            header_row.cols.push(Td {
                content: Rc::new("Plant".to_owned().into()),
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

impl PageComponent for PlantActivityRow {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut cols = vec![Td {
            content: Rc::new(self.activity.date.format(date_format).to_string().into()),
        }];
        if self.include_activity {
            cols.push(Td {
                content: Rc::new(self.activity.activity.clone().into()),
            });
        }

        if self.include_plant {
            cols.push(Td {
                content: Rc::new(self.activity.plant.clone().into()),
            })
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
impl From<(&[Plant], bool, bool)> for PlantActivityTable {
    fn from(
        (plants, include_activity, include_plant): (&[Plant], bool, bool),
    ) -> PlantActivityTable {
        let mut activity_rows = vec![];
        for plant in plants {
            let new_rows: Vec<PlantActivityRow> = plant
                .activities
                .iter()
                .map(|log| (log, include_activity, include_plant).into())
                .collect();
            activity_rows.extend(new_rows);
        }
        PlantActivityTable {
            activity_rows,
            include_activity,
            include_plant,
        }
    }
}

impl From<(&[&LogItem], bool, bool)> for PlantActivityTable {
    fn from(
        (logs, include_activity, include_plant): (&[&LogItem], bool, bool),
    ) -> PlantActivityTable {
        PlantActivityTable {
            activity_rows: logs
                .iter()
                .cloned()
                .map(|x| (x, include_activity, include_plant).into())
                .collect(),
            include_activity,
            include_plant,
        }
    }
}
impl From<(&LogItem, bool, bool)> for PlantActivityRow {
    fn from((log, include_activity, include_plant): (&LogItem, bool, bool)) -> PlantActivityRow {
        PlantActivityRow {
            activity: log.clone(),
            include_activity,
            include_plant,
        }
    }
}
