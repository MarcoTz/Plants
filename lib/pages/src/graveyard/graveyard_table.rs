use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{HtmlElement, Table, Td, Tr},
};
use plants::graveyard::GraveyardPlant;
use std::rc::Rc;

pub struct GraveyardTable {
    plants: Vec<GraveyardPlant>,
}

impl PageComponent for GraveyardTable {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut table_rows = vec![Tr {
            attributes: vec![Attribute::Class(vec!["header_row".to_owned()])],
            cols: vec![
                Td {
                    content: Rc::new("Name".to_owned().into()),
                },
                Td {
                    content: Rc::new("Species".to_owned().into()),
                },
                Td {
                    content: Rc::new("Planted Date".to_owned().into()),
                },
                Td {
                    content: Rc::new("Died Date".to_owned().into()),
                },
                Td {
                    content: Rc::new("Cause of Death".to_owned().into()),
                },
            ],
        }
        .into()];
        for plant in self.plants.iter() {
            let new_row = Tr {
                attributes: vec![],
                cols: vec![
                    Td {
                        content: Rc::new(plant.name.clone().into()),
                    },
                    Td {
                        content: Rc::new(plant.species.clone().into()),
                    },
                    Td {
                        content: Rc::new(plant.planted.format(date_format).to_string().into()),
                    },
                    Td {
                        content: Rc::new(plant.died.format(date_format).to_string().into()),
                    },
                    Td {
                        content: Rc::new(plant.reason.clone().into()),
                    },
                ],
            }
            .into();
            table_rows.push(new_row);
        }
        Table {
            attributes: vec![],
            rows: table_rows,
        }
        .into()
    }
}

impl From<&[GraveyardPlant]> for GraveyardTable {
    fn from(plants: &[GraveyardPlant]) -> GraveyardTable {
        let mut gr_plants = vec![];
        gr_plants.extend_from_slice(plants);
        GraveyardTable { plants: gr_plants }
    }
}
