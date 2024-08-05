use super::super::{
    super::html::{
        attribute::Attribute,
        html_element::HtmlElement,
        table::{Table, Td, Tr},
    },
    page::PageComponent,
};
use plants::log_item::LogItem;
use std::rc::Rc;

struct PlantActivityRow {
    activity: LogItem,
    date_format: String,
}
pub struct PlantActivityTable {
    activity_rows: Vec<PlantActivityRow>,
    include_activity: bool,
}

impl PageComponent for PlantActivityTable {
    fn render(&self) -> HtmlElement {
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
            table_rows.push(activity_row.render(self.include_activity));
        }

        Table { rows: table_rows }.into()
    }
}

impl PlantActivityRow {
    pub fn render(&self, include_activity: bool) -> Tr {
        let mut cols = vec![Td {
            content: Rc::new(
                self.activity
                    .date
                    .format(&self.date_format)
                    .to_string()
                    .into(),
            ),
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
