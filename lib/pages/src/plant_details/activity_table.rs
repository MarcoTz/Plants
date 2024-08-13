use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    html_element::HtmlElement,
    table::{Table, Td, Tr},
};
use plants::log_item::LogItem;
use std::rc::Rc;

struct ActivityRow {
    activity: LogItem,
    include_activity: bool,
}
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
