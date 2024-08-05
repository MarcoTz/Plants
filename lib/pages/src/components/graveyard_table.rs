use super::page_component::PageComponent;
use html::{
    attribute::Attribute,
    html_element::HtmlElement,
    table::{Table, Td, Tr},
};
use plants::graveyard::GraveyardPlant;
use std::rc::Rc;

pub struct GraveyardTable {
    plants: Vec<GraveyardPlant>,
    date_format: String,
}

impl PageComponent for GraveyardTable {
    fn render(&self) -> HtmlElement {
        let mut table_rows = vec![Tr {
            attributes: vec![Attribute::Class("header_row".to_owned())],
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
        }];
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
                        content: Rc::new(
                            plant.planted.format(&self.date_format).to_string().into(),
                        ),
                    },
                    Td {
                        content: Rc::new(plant.died.format(&self.date_format).to_string().into()),
                    },
                    Td {
                        content: Rc::new(plant.reason.clone().into()),
                    },
                ],
            };
            table_rows.push(new_row);
        }
        Table { rows: table_rows }.into()
    }
}
