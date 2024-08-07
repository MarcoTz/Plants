use super::page_component::PageComponent;
use html::{
    attribute::Attribute,
    html_element::HtmlElement,
    table::{Table, Td, Tr},
};
use plants::log_item::LogItem;
use std::rc::Rc;

struct PlantActivityRow {
    activity: LogItem,
}
pub struct PlantActivityTable {
    activity_rows: Vec<PlantActivityRow>,
    include_activity: bool,
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

        header_row.cols.push(Td {
            content: Rc::new("Note".to_owned().into()),
        });

        let mut table_rows = vec![header_row];
        for activity_row in self.activity_rows.iter() {
            table_rows.push(activity_row.render(date_format, self.include_activity));
        }

        Table { rows: table_rows }.into()
    }
}

impl PlantActivityRow {
    pub fn render(&self, date_format: &str, include_activity: bool) -> Tr {
        let mut cols = vec![Td {
            content: Rc::new(self.activity.date.format(date_format).to_string().into()),
        }];
        if include_activity {
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
    }
}
impl From<(&[&LogItem], bool)> for PlantActivityTable {
    fn from((logs, include_activity): (&[&LogItem], bool)) -> PlantActivityTable {
        PlantActivityTable {
            activity_rows: logs.iter().cloned().map(|x| x.into()).collect(),
            include_activity,
        }
    }
}
impl From<&LogItem> for PlantActivityRow {
    fn from(log: &LogItem) -> PlantActivityRow {
        PlantActivityRow {
            activity: log.clone(),
        }
    }
}
