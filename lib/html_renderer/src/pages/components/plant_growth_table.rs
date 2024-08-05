use super::super::{
    super::html_components::{
        component::HtmlComponent,
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
    fn render(&self) -> HtmlComponent {
        let header_row = Tr {
            id: Some("header_row".to_owned()),
            class: None,
            cols: vec![
                Td {
                    contents: Rc::new("Date".to_owned().into()),
                },
                Td {
                    contents: Rc::new("Height".to_owned().into()),
                },
                Td {
                    contents: Rc::new("Width".to_owned().into()),
                },
                Td {
                    contents: Rc::new("Note".to_owned().into()),
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
                contents: Rc::new(
                    self.growth
                        .date
                        .format(&self.date_format)
                        .to_string()
                        .into(),
                ),
            },
            Td {
                contents: Rc::new(self.growth.height_cm.to_string().into()),
            },
            Td {
                contents: Rc::new(self.growth.width_cm.to_string().into()),
            },
            Td {
                contents: Rc::new(self.growth.note.clone().unwrap_or("".to_owned()).into()),
            },
        ];

        Tr {
            id: None,
            class: None,
            cols,
        }
    }
}
