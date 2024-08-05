use super::super::{
    super::html::{
        attribute::Attribute,
        html_element::HtmlElement,
        table::{Table, Td, Tr},
    },
    page::PageComponent,
};
use plants::growth_item::GrowthItem;
use std::rc::Rc;

struct PlantGrowthRow {
    growth: GrowthItem,
    date_format: String,
}

pub struct PlantGrowthTable {
    growth_rows: Vec<PlantGrowthRow>,
}

impl PageComponent for PlantGrowthTable {
    fn render(&self) -> HtmlElement {
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
                    content: Rc::new("Note".to_owned().into()),
                },
            ],
        };

        let mut table_rows = vec![header_row];
        for growth_row in self.growth_rows.iter() {
            table_rows.push(growth_row.render());
        }
        Table { rows: table_rows }.into()
    }
}

impl PlantGrowthRow {
    fn render(&self) -> Tr {
        let cols = vec![
            Td {
                content: Rc::new(
                    self.growth
                        .date
                        .format(&self.date_format)
                        .to_string()
                        .into(),
                ),
            },
            Td {
                content: Rc::new(self.growth.height_cm.to_string().into()),
            },
            Td {
                content: Rc::new(self.growth.width_cm.to_string().into()),
            },
            Td {
                content: Rc::new(self.growth.note.clone().unwrap_or("".to_owned()).into()),
            },
        ];

        Tr {
            attributes: vec![],
            cols,
        }
    }
}
