use super::super::{
    super::html_components::{
        component::HtmlComponent,
        table::{Table, Td, Tr},
    },
    page::PageComponent,
};
use plants::graveyard::GraveyardPlant;
use std::rc::Rc;

pub struct GraveyardTable {
    plants: Vec<GraveyardPlant>,
    date_format: String,
}

impl PageComponent for GraveyardTable {
    fn render(&self) -> HtmlComponent {
        let mut table_rows = vec![Tr {
            class: Some("header_row".to_owned()),
            cols: vec![
                Td {
                    contents: Rc::new("Name".to_owned().into()),
                },
                Td {
                    contents: Rc::new("Species".to_owned().into()),
                },
                Td {
                    contents: Rc::new("Planted Date".to_owned().into()),
                },
                Td {
                    contents: Rc::new("Died Date".to_owned().into()),
                },
                Td {
                    contents: Rc::new("Cause of Death".to_owned().into()),
                },
            ],
        }];
        for plant in self.plants.iter() {
            let new_row = Tr {
                class: None,
                cols: vec![
                    Td {
                        contents: Rc::new(plant.name.clone().into()),
                    },
                    Td {
                        contents: Rc::new(plant.species.clone().into()),
                    },
                    Td {
                        contents: Rc::new(
                            plant.planted.format(&self.date_format).to_string().into(),
                        ),
                    },
                    Td {
                        contents: Rc::new(plant.died.format(&self.date_format).to_string().into()),
                    },
                    Td {
                        contents: Rc::new(plant.reason.clone().into()),
                    },
                ],
            };
            table_rows.push(new_row);
        }
        Table { rows: table_rows }.into()
    }
}
