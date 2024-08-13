use crate::components::page_component::PageComponent;
use html::{
    attribute::Attribute,
    html_element::HtmlElement,
    table::{Table, Td, Tr},
};
use plants::growth_item::GrowthItem;
use std::rc::Rc;

struct GrowthRow {
    growth: GrowthItem,
}

pub struct GrowthTable {
    growth_rows: Vec<GrowthRow>,
}

impl PageComponent for GrowthTable {
    fn render(&self, date_format: &str) -> HtmlElement {
        let header_row = Tr {
            attributes: vec![Attribute::Id("header_row".to_owned())],
            cols: vec![
                Td {
                    content: Rc::new("Date".to_owned().into()),
                },
                Td {
                    content: Rc::new("Height".to_owned().into()),
                },
                Td {
                    content: Rc::new("Width".to_owned().into()),
                },
                Td {
                    content: Rc::new("Health".to_owned().into()),
                },
                Td {
                    content: Rc::new("Note".to_owned().into()),
                },
            ],
        };

        let mut table_rows = vec![header_row.into()];
        for growth_row in self.growth_rows.iter() {
            table_rows.push(growth_row.render(date_format));
        }
        Table {
            attributes: vec![],
            rows: table_rows,
        }
        .into()
    }
}

impl PageComponent for GrowthRow {
    fn render(&self, date_format: &str) -> HtmlElement {
        let cols = vec![
            Td {
                content: Rc::new(self.growth.date.format(date_format).to_string().into()),
            },
            Td {
                content: Rc::new(self.growth.height_cm.to_string().into()),
            },
            Td {
                content: Rc::new(self.growth.width_cm.to_string().into()),
            },
            Td {
                content: Rc::new(self.growth.health.to_string().into()),
            },
            Td {
                content: Rc::new(self.growth.note.clone().unwrap_or("".to_owned()).into()),
            },
        ];

        Tr {
            attributes: vec![],
            cols,
        }
        .into()
    }
}

impl From<&GrowthItem> for GrowthRow {
    fn from(growth: &GrowthItem) -> GrowthRow {
        GrowthRow {
            growth: growth.clone(),
        }
    }
}

impl From<&[GrowthItem]> for GrowthTable {
    fn from(growth: &[GrowthItem]) -> GrowthTable {
        GrowthTable {
            growth_rows: growth.iter().map(|x| x.into()).collect(),
        }
    }
}
