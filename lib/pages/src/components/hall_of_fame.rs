use super::super::errors::Error;
use super::page_component::PageComponent;
use html::{
    attribute::Attribute,
    div::Div,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
    table::{Table, Td, Tr},
};
use plants::{plant, plant::Plant};
use std::rc::Rc;

#[derive(Clone)]
struct HallOfFameItem {
    plant_name: String,
    plant_value: f32,
    plant_unit: String,
}
struct HallOfFameTable {
    title: String,
    plants: Vec<HallOfFameItem>,
}
pub struct HallOfFame {
    tallest: HallOfFameTable,
    shortest: HallOfFameTable,
    widest: HallOfFameTable,
    thinnest: HallOfFameTable,
    fastest_growing: HallOfFameTable,
    slowest_growing: HallOfFameTable,
    oldest: HallOfFameTable,
    youngest: HallOfFameTable,
}

impl PageComponent for HallOfFame {
    fn render(&self, date_format: &str) -> HtmlElement {
        let hall_of_fame_header = Headline {
            size: HeaderSize::H1,
            content: Rc::new("Hall Of Fame".to_owned().into()),
        }
        .into();
        let hall_of_fame_items = vec![
            self.tallest.render(date_format),
            self.shortest.render(date_format),
            self.widest.render(date_format),
            self.thinnest.render(date_format),
            self.fastest_growing.render(date_format),
            self.slowest_growing.render(date_format),
            self.oldest.render(date_format),
            self.youngest.render(date_format),
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

impl TryFrom<&[Plant]> for HallOfFame {
    type Error = Error;
    fn try_from(plants: &[Plant]) -> Result<HallOfFame, Self::Error> {
        let by_height = plant::sort_height(plants)?;
        let by_height_items: Vec<HallOfFameItem> = by_height
            .iter()
            .map(|(val, plant)| HallOfFameItem {
                plant_name: plant.name.clone(),
                plant_value: val.to_owned(),
                plant_unit: "cm".to_owned(),
            })
            .collect();
        let mut by_height_items_rev = by_height_items.clone();
        by_height_items_rev.reverse();
        let by_width = plant::sort_width(plants)?;
        let by_width_items: Vec<HallOfFameItem> = by_width
            .iter()
            .map(|(val, plant)| HallOfFameItem {
                plant_name: plant.name.clone(),
                plant_value: val.to_owned(),
                plant_unit: "cm".to_owned(),
            })
            .collect();
        let mut by_width_items_rev = by_width_items.clone();
        by_width_items_rev.reverse();
        let by_speed = plant::sort_speed(plants)?;
        let by_speed_items: Vec<HallOfFameItem> = by_speed
            .iter()
            .map(|(val, plant)| HallOfFameItem {
                plant_name: plant.name.clone(),
                plant_value: val.to_owned(),
                plant_unit: "cm/day".to_owned(),
            })
            .collect();
        let mut by_speed_items_rev = by_speed_items.clone();
        by_speed_items_rev.reverse();
        let by_age = plant::sort_age(plants)?;
        let by_age_items: Vec<HallOfFameItem> = by_age
            .iter()
            .map(|(val, plant)| HallOfFameItem {
                plant_name: plant.name.clone(),
                plant_value: val.to_owned(),
                plant_unit: "days".to_owned(),
            })
            .collect();
        let mut by_age_items_rev = by_age_items.clone();
        by_age_items_rev.reverse();
        Ok(HallOfFame {
            tallest: ("Tallest".to_owned(), by_height_items).into(),
            shortest: ("Shortest".to_owned(), by_height_items_rev).into(),
            widest: ("Widest".to_owned(), by_width_items).into(),
            thinnest: ("Thinnest".to_owned(), by_width_items_rev).into(),
            fastest_growing: ("Fastest Growing".to_owned(), by_speed_items).into(),
            slowest_growing: ("Slowest Growing".to_owned(), by_speed_items_rev).into(),
            oldest: ("Oldest".to_owned(), by_age_items).into(),
            youngest: ("Youngest".to_owned(), by_age_items_rev).into(),
        })
    }
}

impl PageComponent for HallOfFameTable {
    fn render(&self, _: &str) -> HtmlElement {
        let header = Headline {
            size: HeaderSize::H3,
            content: Rc::new(self.title.clone().into()),
        }
        .into();
        let mut fame_table = Table { rows: vec![] };
        for (i, fame_item) in self.plants.iter().enumerate() {
            let ind_col = Td {
                content: Rc::new((i + 1).to_string().into()),
            };
            let name_col = Td {
                content: Rc::new(fame_item.plant_name.clone().into()),
            };
            let value_col = Td {
                content: Rc::new(
                    (format!("{:.2}", fame_item.plant_value).replace(".00", "")
                        + " "
                        + &fame_item.plant_unit)
                        .into(),
                ),
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

impl From<(String, Vec<HallOfFameItem>)> for HallOfFameTable {
    fn from((s, plants): (String, Vec<HallOfFameItem>)) -> HallOfFameTable {
        HallOfFameTable {
            title: s,
            plants: plants.iter().take(10).cloned().collect(),
        }
    }
}
