use super::super::{
    super::html_components::{
        attribute::Attribute,
        component::HtmlComponent,
        div::Div,
        headline::{HeaderSize, Headline},
        table::{Table, Td, Tr},
    },
    page::PageComponent,
};
use std::rc::Rc;

struct HallOfFameItem {
    title: String,
    plants: Vec<(String, String)>,
}
pub struct HallOfFame {
    tallest: HallOfFameItem,
    shortest: HallOfFameItem,
    widest: HallOfFameItem,
    thinnest: HallOfFameItem,
    fastest_growing: HallOfFameItem,
    slowest_growing: HallOfFameItem,
    oldest: HallOfFameItem,
    youngest: HallOfFameItem,
}

impl PageComponent for HallOfFame {
    fn render(&self) -> HtmlComponent {
        let hall_of_fame_header = Headline {
            size: HeaderSize::H1,
            content: Rc::new("Hall Of Fame".to_owned().into()),
        }
        .into();
        let hall_of_fame_items = vec![
            self.tallest.render(),
            self.shortest.render(),
            self.widest.render(),
            self.thinnest.render(),
            self.fastest_growing.render(),
            self.slowest_growing.render(),
            self.oldest.render(),
            self.youngest.render(),
        ];

        vec![
            hall_of_fame_header,
            Div {
                attributes: vec![Attribute::Id("hall_of_fame".to_owned())],
                content: Rc::new(hall_of_fame_items.into()),
            }
            .into(),
        ]
        .into()
    }
}

impl PageComponent for HallOfFameItem {
    fn render(&self) -> HtmlComponent {
        let header = Headline {
            size: HeaderSize::H3,
            content: Rc::new(self.title.clone().into()),
        }
        .into();
        let mut fame_table = Table { rows: vec![] };
        for (i, (plant_name, plant_value)) in self.plants.iter().enumerate() {
            let ind_col = Td {
                content: Rc::new(i.to_string().into()),
            };
            let name_col = Td {
                content: Rc::new(plant_name.clone().into()),
            };
            let value_col = Td {
                content: Rc::new(plant_value.clone().into()),
            };
            let new_row = Tr {
                attributes: vec![],
                cols: vec![ind_col, name_col, value_col],
            };
            fame_table.rows.push(new_row)
        }
        let div_content = Rc::new(vec![header, fame_table.into()].into());
        Div {
            attributes: vec![Attribute::Class("hall_of_fame_item".to_owned())],
            content: div_content,
        }
        .into()
    }
}
