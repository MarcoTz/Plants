use crate::{errors::Error, page::PageComponent, shared::plant_link::PlantLink};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement, Table, Td, Tr},
};
use plants::{plant, plant::Plant};
use std::rc::Rc;

#[derive(Clone)]
struct HallOfFameItem {
    plant: PlantLink,
    num: i32,
    value: f32,
    unit: String,
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

impl PageComponent for HallOfFameItem {
    fn render(&self, date_format: &str) -> HtmlElement {
        log::info!("Loading Hall of Fame");
        let value_str = format!("{:.2} {}", self.value, self.unit);
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((self.num.to_string() + ". ").into()),
                },
                Td {
                    content: Rc::new(self.plant.render(date_format)),
                },
                Td {
                    content: Rc::new(value_str.into()),
                },
            ],
        }
        .into()
    }
}

impl PageComponent for HallOfFameTable {
    fn render(&self, date_format: &str) -> HtmlElement {
        let hall_of_fame_header = Headline {
            attributes: vec![],
            size: HeaderSize::H3,
            content: Rc::new(self.title.clone().into()),
        }
        .into();

        let mut hall_of_fame_rows = vec![];
        for item in self.plants.iter() {
            hall_of_fame_rows.push(item.render(date_format));
        }

        let hall_of_fame_table = Table {
            attributes: vec![],
            rows: hall_of_fame_rows,
        }
        .into();

        Div {
            attributes: vec![Attribute::Id("hall_of_fame_table".to_owned())],
            content: Rc::new(vec![hall_of_fame_header, hall_of_fame_table].into()),
        }
        .into()
    }
}

impl PageComponent for HallOfFame {
    fn render(&self, date_format: &str) -> HtmlElement {
        let hall_of_fame_header = Headline {
            attributes: vec![],
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
                attributes: vec![
                    Attribute::Id("hall_of_fame".to_owned()),
                    Attribute::Class(vec![
                        "flex_container".to_owned(),
                        "alternating_children".to_owned(),
                    ]),
                ],
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
            .enumerate()
            .map(|(i, (val, plant))| HallOfFameItem {
                plant: (*plant, "plants/").into(),
                num: i as i32 + 1,
                value: val.to_owned(),
                unit: "cm".to_owned(),
            })
            .collect();
        let mut by_height_items_rev: Vec<HallOfFameItem> = by_height_items.clone();
        by_height_items_rev.reverse();
        for item in by_height_items_rev.iter_mut() {
            item.num = (plants.len() as i32) - item.num + 1;
        }
        let by_width = plant::sort_width(plants)?;
        let by_width_items: Vec<HallOfFameItem> = by_width
            .iter()
            .enumerate()
            .map(|(i, (val, plant))| HallOfFameItem {
                plant: (*plant, "plants/").into(),
                num: i as i32 + 1,
                value: val.to_owned(),
                unit: "cm".to_owned(),
            })
            .collect();
        let mut by_width_items_rev = by_width_items.clone();
        by_width_items_rev.reverse();
        for item in by_width_items_rev.iter_mut() {
            item.num = (plants.len() as i32) - item.num + 1;
        }

        let by_speed = plant::sort_speed(plants)?;
        let by_speed_items: Vec<HallOfFameItem> = by_speed
            .iter()
            .enumerate()
            .map(|(i, (val, plant))| HallOfFameItem {
                plant: (*plant, "plants").into(),
                value: val.to_owned(),
                num: i as i32 + 1,
                unit: "cm/day".to_owned(),
            })
            .collect();
        let mut by_speed_items_rev = by_speed_items.clone();
        by_speed_items_rev.reverse();
        for item in by_speed_items_rev.iter_mut() {
            item.num = (plants.len() as i32) - item.num + 1;
        }

        let by_age = plant::sort_age(plants)?;
        let by_age_items: Vec<HallOfFameItem> = by_age
            .iter()
            .enumerate()
            .map(|(i, (val, plant))| HallOfFameItem {
                plant: (*plant, "plants").into(),
                num: i as i32 + 1,
                value: val.to_owned(),
                unit: "days".to_owned(),
            })
            .collect();
        let mut by_age_items_rev = by_age_items.clone();
        by_age_items_rev.reverse();
        for item in by_age_items_rev.iter_mut() {
            item.num = (plants.len() as i32) - item.num + 1;
        }

        Ok(HallOfFame {
            tallest: ("Tallest".to_owned(), by_height_items_rev).into(),
            shortest: ("Shortest".to_owned(), by_height_items).into(),
            widest: ("Widest".to_owned(), by_width_items_rev).into(),
            thinnest: ("Thinnest".to_owned(), by_width_items).into(),
            fastest_growing: ("Fastest Growing".to_owned(), by_speed_items_rev).into(),
            slowest_growing: ("Slowest Growing".to_owned(), by_speed_items).into(),
            oldest: ("Oldest".to_owned(), by_age_items_rev).into(),
            youngest: ("Youngest".to_owned(), by_age_items).into(),
        })
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
