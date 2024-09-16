use crate::{errors::Error, page::PageComponent, shared::plant_link::PlantLink};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement, Table, Td, Tr},
};
use plants::{plant, plant::Plant};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
struct HallOfFameItem {
    plant: PlantLink,
    num: i32,
    value: f32,
    unit: String,
}

#[derive(Debug, PartialEq)]
struct HallOfFameTable {
    title: String,
    plants: Vec<HallOfFameItem>,
}

#[derive(Debug, PartialEq)]
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
        // if sort height succeeds, the other sorts cannot fail
        let by_height = plant::sort_height(plants)?;
        let by_height_items: Vec<HallOfFameItem> = by_height
            .iter()
            .enumerate()
            .map(|(i, (val, plant))| HallOfFameItem {
                plant: (*plant, "plants").into(),
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
        let by_width = plant::sort_width(plants).unwrap();
        let by_width_items: Vec<HallOfFameItem> = by_width
            .iter()
            .enumerate()
            .map(|(i, (val, plant))| HallOfFameItem {
                plant: (*plant, "plants").into(),
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

        let by_speed = plant::sort_speed(plants).unwrap();
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

        let by_age = plant::sort_age(plants).unwrap();
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

#[cfg(test)]
mod hall_of_fame_tests {
    use super::{HallOfFame, HallOfFameItem, HallOfFameTable, PageComponent};
    use crate::test_common::{
        example_plant1, example_plant2, example_plant3, example_plantlink1, example_plantlink2,
        example_plantlink3, sample_date1, sample_date2, sample_date3, DATE_FORMAT,
    };
    use chrono::Local;
    use html::{
        attribute::Attribute,
        elements::{Div, HeaderSize, Headline, HtmlElement, Table, Td, Tr},
    };
    use std::rc::Rc;

    fn example_fame_item1_height(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink1(),
            num: place,
            value: 100.0,
            unit: "cm".to_owned(),
        }
    }

    fn example_fame_item1_height_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").into()),
                },
                Td {
                    content: Rc::new(example_plantlink1().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new("100.00 cm".to_owned().into()),
                },
            ],
        }
        .into()
    }

    fn example_fame_item1_width(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink1(),
            num: place,
            value: 75.0,
            unit: "cm".to_owned(),
        }
    }

    fn example_fame_item1_width_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").to_owned().into()),
                },
                Td {
                    content: Rc::new(example_plantlink1().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new("75.00 cm".to_owned().into()),
                },
            ],
        }
        .into()
    }

    fn example_fame_item1_speed(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink1(),
            num: place,
            value: 50.0,
            unit: "cm/day".to_owned(),
        }
    }

    fn example_fame_item1_speed_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").into()),
                },
                Td {
                    content: Rc::new(example_plantlink1().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new("50.00 cm/day".to_owned().into()),
                },
            ],
        }
        .into()
    }

    fn example_fame_item1_age(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink1(),
            num: place,
            value: (Local::now().date_naive() - sample_date1()).num_days() as f32,
            unit: "days".to_owned(),
        }
    }

    fn example_fame_item1_age_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").into()),
                },
                Td {
                    content: Rc::new(example_plantlink1().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new(
                        ((Local::now().date_naive() - sample_date1())
                            .num_days()
                            .to_string()
                            + ".00 days")
                            .into(),
                    ),
                },
            ],
        }
        .into()
    }

    fn example_fame_item2_height(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink2(),
            num: place,
            value: 75.3,
            unit: "cm".to_owned(),
        }
    }

    fn example_fame_item2_height_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").into()),
                },
                Td {
                    content: Rc::new(example_plantlink2().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new("75.30 cm".to_owned().into()),
                },
            ],
        }
        .into()
    }

    fn example_fame_item2_width(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink2(),
            num: place,
            value: 98.5,
            unit: "cm".to_owned(),
        }
    }

    fn example_fame_item2_width_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").into()),
                },
                Td {
                    content: Rc::new(example_plantlink2().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new("98.50 cm".to_owned().into()),
                },
            ],
        }
        .into()
    }

    fn example_fame_item2_speed(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink2(),
            num: place,
            value: 65.75,
            unit: "cm/day".to_owned(),
        }
    }

    fn example_fame_item2_speed_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").into()),
                },
                Td {
                    content: Rc::new(example_plantlink2().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new("65.75 cm/day".to_owned().into()),
                },
            ],
        }
        .into()
    }

    fn example_fame_item2_age(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink2(),
            num: place,
            value: (Local::now().date_naive() - sample_date2()).num_days() as f32,
            unit: "days".to_owned(),
        }
    }

    fn example_fame_item2_age_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").to_owned().into()),
                },
                Td {
                    content: Rc::new(example_plantlink2().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new(
                        ((Local::now().date_naive() - sample_date2())
                            .num_days()
                            .to_string()
                            + ".00 days")
                            .into(),
                    ),
                },
            ],
        }
        .into()
    }

    fn example_fame_item3_height(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink3(),
            num: place,
            value: 34.2,
            unit: "cm".to_owned(),
        }
    }

    fn example_fame_item3_height_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").into()),
                },
                Td {
                    content: Rc::new(example_plantlink3().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new("34.20 cm".to_owned().into()),
                },
            ],
        }
        .into()
    }

    fn example_fame_item3_width(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink3(),
            num: place,
            value: 83.4,
            unit: "cm".to_owned(),
        }
    }

    fn example_fame_item3_width_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").into()),
                },
                Td {
                    content: Rc::new(example_plantlink3().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new("83.40 cm".to_owned().into()),
                },
            ],
        }
        .into()
    }

    fn example_fame_item3_speed(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink3(),
            num: place,
            value: 54.15,
            unit: "cm/day".to_owned(),
        }
    }

    fn example_fame_item3_speed_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").into()),
                },
                Td {
                    content: Rc::new(example_plantlink3().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new("54.15 cm/day".to_owned().into()),
                },
            ],
        }
        .into()
    }

    fn example_fame_item3_age(place: i32) -> HallOfFameItem {
        HallOfFameItem {
            plant: example_plantlink3(),
            num: place,
            value: (Local::now().date_naive() - sample_date3()).num_days() as f32,
            unit: "days".to_owned(),
        }
    }

    fn example_fame_item3_age_rendered(place: i32) -> HtmlElement {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new((place.to_string() + ". ").to_owned().into()),
                },
                Td {
                    content: Rc::new(example_plantlink3().render(DATE_FORMAT)),
                },
                Td {
                    content: Rc::new(
                        ((Local::now().date_naive() - sample_date3())
                            .num_days()
                            .to_string()
                            + ".00 days")
                            .into(),
                    ),
                },
            ],
        }
        .into()
    }

    fn example_table_tallest() -> HallOfFameTable {
        HallOfFameTable {
            title: "Tallest".to_owned(),
            plants: vec![
                example_fame_item1_height(1),
                example_fame_item2_height(2),
                example_fame_item3_height(3),
            ],
        }
    }

    fn example_table_tallest_rendered() -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("hall_of_fame_table".to_owned())],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H3,
                        content: Rc::new("Tallest".to_owned().into()),
                    }
                    .into(),
                    Table {
                        attributes: vec![],
                        rows: vec![
                            example_fame_item1_height_rendered(1),
                            example_fame_item2_height_rendered(2),
                            example_fame_item3_height_rendered(3),
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }

    fn example_table_shortest() -> HallOfFameTable {
        HallOfFameTable {
            title: "Shortest".to_owned(),
            plants: vec![
                example_fame_item3_height(1),
                example_fame_item2_height(2),
                example_fame_item1_height(3),
            ],
        }
    }

    fn example_table_shortest_rendered() -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("hall_of_fame_table".to_owned())],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H3,
                        content: Rc::new("Shortest".to_owned().into()),
                    }
                    .into(),
                    Table {
                        attributes: vec![],
                        rows: vec![
                            example_fame_item3_height_rendered(1),
                            example_fame_item2_height_rendered(2),
                            example_fame_item1_height_rendered(3),
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }

    fn example_table_widest() -> HallOfFameTable {
        HallOfFameTable {
            title: "Widest".to_owned(),
            plants: vec![
                example_fame_item2_width(1),
                example_fame_item3_width(2),
                example_fame_item1_width(3),
            ],
        }
    }

    fn example_table_widest_rendered() -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("hall_of_fame_table".to_owned())],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H3,
                        content: Rc::new("Widest".to_owned().into()),
                    }
                    .into(),
                    Table {
                        attributes: vec![],
                        rows: vec![
                            example_fame_item2_width_rendered(1),
                            example_fame_item3_width_rendered(2),
                            example_fame_item1_width_rendered(3),
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }

    fn example_table_thinnest() -> HallOfFameTable {
        HallOfFameTable {
            title: "Thinnest".to_owned(),
            plants: vec![
                example_fame_item1_width(1),
                example_fame_item3_width(2),
                example_fame_item2_width(3),
            ],
        }
    }

    fn example_table_thinnest_rendered() -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("hall_of_fame_table".to_owned())],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H3,
                        content: Rc::new("Thinnest".to_owned().into()),
                    }
                    .into(),
                    Table {
                        attributes: vec![],
                        rows: vec![
                            example_fame_item1_width_rendered(1),
                            example_fame_item3_width_rendered(2),
                            example_fame_item2_width_rendered(3),
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }

    fn example_table_fastest() -> HallOfFameTable {
        HallOfFameTable {
            title: "Fastest Growing".to_owned(),
            plants: vec![
                example_fame_item2_speed(1),
                example_fame_item3_speed(2),
                example_fame_item1_speed(3),
            ],
        }
    }

    fn example_table_fastest_rendered() -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("hall_of_fame_table".to_owned())],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H3,
                        content: Rc::new("Fastest Growing".to_owned().into()),
                    }
                    .into(),
                    Table {
                        attributes: vec![],
                        rows: vec![
                            example_fame_item2_speed_rendered(1),
                            example_fame_item3_speed_rendered(2),
                            example_fame_item1_speed_rendered(3),
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }

    fn example_table_slowest() -> HallOfFameTable {
        HallOfFameTable {
            title: "Slowest Growing".to_owned(),
            plants: vec![
                example_fame_item1_speed(1),
                example_fame_item3_speed(2),
                example_fame_item2_speed(3),
            ],
        }
    }

    fn example_table_slowest_rendered() -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("hall_of_fame_table".to_owned())],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H3,
                        content: Rc::new("Slowest Growing".to_owned().into()),
                    }
                    .into(),
                    Table {
                        attributes: vec![],
                        rows: vec![
                            example_fame_item1_speed_rendered(1),
                            example_fame_item3_speed_rendered(2),
                            example_fame_item2_speed_rendered(3),
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }

    fn example_table_oldest() -> HallOfFameTable {
        HallOfFameTable {
            title: "Oldest".to_owned(),
            plants: vec![
                example_fame_item1_age(1),
                example_fame_item2_age(2),
                example_fame_item3_age(3),
            ],
        }
    }

    fn example_table_oldest_rendered() -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("hall_of_fame_table".to_owned())],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H3,
                        content: Rc::new("Oldest".to_owned().into()),
                    }
                    .into(),
                    Table {
                        attributes: vec![],
                        rows: vec![
                            example_fame_item1_age_rendered(1),
                            example_fame_item2_age_rendered(2),
                            example_fame_item3_age_rendered(3),
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }

    fn example_table_youngest() -> HallOfFameTable {
        HallOfFameTable {
            title: "Youngest".to_owned(),
            plants: vec![
                example_fame_item3_age(1),
                example_fame_item2_age(2),
                example_fame_item1_age(3),
            ],
        }
    }

    fn example_table_youngest_rendered() -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("hall_of_fame_table".to_owned())],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H3,
                        content: Rc::new("Youngest".to_owned().into()),
                    }
                    .into(),
                    Table {
                        attributes: vec![],
                        rows: vec![
                            example_fame_item3_age_rendered(1),
                            example_fame_item2_age_rendered(2),
                            example_fame_item1_age_rendered(3),
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }

    fn example_hall_of_fame() -> HallOfFame {
        HallOfFame {
            tallest: example_table_tallest(),
            shortest: example_table_shortest(),
            widest: example_table_widest(),
            thinnest: example_table_thinnest(),
            fastest_growing: example_table_fastest(),
            slowest_growing: example_table_slowest(),
            oldest: example_table_oldest(),
            youngest: example_table_youngest(),
        }
    }

    #[test]
    fn render_hall_of_fame() {
        let result = example_hall_of_fame().render(DATE_FORMAT);
        let expected = vec![
            Headline {
                attributes: vec![],
                size: HeaderSize::H1,
                content: Rc::new("Hall Of Fame".to_owned().into()),
            }
            .into(),
            Div {
                attributes: vec![
                    Attribute::Id("hall_of_fame".to_owned()),
                    Attribute::Class(vec![
                        "flex_container".to_owned(),
                        "alternating_children".to_owned(),
                    ]),
                ],
                content: Rc::new(
                    vec![
                        example_table_tallest_rendered(),
                        example_table_shortest_rendered(),
                        example_table_widest_rendered(),
                        example_table_thinnest_rendered(),
                        example_table_fastest_rendered(),
                        example_table_slowest_rendered(),
                        example_table_oldest_rendered(),
                        example_table_youngest_rendered(),
                    ]
                    .into(),
                ),
            }
            .into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item1_height() {
        let result = example_fame_item1_height(1).render(DATE_FORMAT);
        let expected = example_fame_item1_height_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item1_width() {
        let result = example_fame_item1_width(1).render(DATE_FORMAT);
        let expected = example_fame_item1_width_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item1_speed() {
        let result = example_fame_item1_speed(1).render(DATE_FORMAT);
        let expected = example_fame_item1_speed_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item1_age() {
        let result = example_fame_item1_age(1).render(DATE_FORMAT);
        let expected = example_fame_item1_age_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item2_height() {
        let result = example_fame_item2_height(1).render(DATE_FORMAT);
        let expected = example_fame_item2_height_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item2_width() {
        let result = example_fame_item2_width(1).render(DATE_FORMAT);
        let expected = example_fame_item2_width_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item2_speed() {
        let result = example_fame_item2_speed(1).render(DATE_FORMAT);
        let expected = example_fame_item2_speed_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item2_age() {
        let result = example_fame_item2_age(1).render(DATE_FORMAT);
        let expected = example_fame_item2_age_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item3_height() {
        let result = example_fame_item3_height(1).render(DATE_FORMAT);
        let expected = example_fame_item3_height_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item3_width() {
        let result = example_fame_item3_width(1).render(DATE_FORMAT);
        let expected = example_fame_item3_width_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item3_speed() {
        let result = example_fame_item3_speed(1).render(DATE_FORMAT);
        let expected = example_fame_item3_speed_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_item3_age() {
        let result = example_fame_item3_age(1).render(DATE_FORMAT);
        let expected = example_fame_item3_age_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_table_tallest() {
        let result = example_table_tallest().render(DATE_FORMAT);
        let expected = example_table_tallest_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_table_shortest() {
        let result = example_table_shortest().render(DATE_FORMAT);
        let expected = example_table_shortest_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_table_widest() {
        let result = example_table_widest().render(DATE_FORMAT);
        let expected = example_table_widest_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_table_thinnest() {
        let result = example_table_thinnest().render(DATE_FORMAT);
        let expected = example_table_thinnest_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_table_fastest() {
        let result = example_table_fastest().render(DATE_FORMAT);
        let expected = example_table_fastest_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_table_slowest() {
        let result = example_table_slowest().render(DATE_FORMAT);
        let expected = example_table_slowest_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_table_oldest() {
        let result = example_table_oldest().render(DATE_FORMAT);
        let expected = example_table_oldest_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hall_of_fame_table_youngest() {
        let result = example_table_youngest().render(DATE_FORMAT);
        let expected = example_table_youngest_rendered();
        assert_eq!(result, expected)
    }

    #[test]
    fn into_fame_table() {
        let result = HallOfFameTable::from((
            "Tallest".to_owned(),
            vec![
                example_fame_item1_height(1),
                example_fame_item2_height(2),
                example_fame_item3_height(3),
            ],
        ));
        let expected = example_table_tallest();
        assert_eq!(result, expected)
    }

    #[test]
    fn into_hall_of_fame() {
        let result = HallOfFame::try_from(
            vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
        )
        .unwrap();
        let expected = example_hall_of_fame();
        assert_eq!(result, expected)
    }

    #[test]
    fn into_hall_of_fame_height_err() {
        let mut plant = example_plant1();
        plant.growth = vec![];
        let result = HallOfFame::try_from(vec![plant].as_slice());
        assert!(result.is_err())
    }
}
