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
        let mut by_height_rev = by_height.clone();
        by_height_rev.reverse();
        let by_width = plant::sort_width(plants)?;
        let mut by_width_rev = by_width.clone();
        by_width_rev.reverse();
        let by_speed = plant::sort_speed(plants)?;
        let mut by_speed_rev = by_speed.clone();
        by_speed_rev.reverse();
        let by_age = plant::sort_age(plants)?;
        let mut by_age_rev = by_age.clone();
        by_age_rev.reverse();
        Ok(HallOfFame {
            tallest: ("Tallest".to_owned(), by_height).into(),
            shortest: ("Shortest".to_owned(), by_height_rev).into(),
            widest: ("Widest".to_owned(), by_width).into(),
            thinnest: ("Thinnest".to_owned(), by_width_rev).into(),
            fastest_growing: ("Fastest Growing".to_owned(), by_speed).into(),
            slowest_growing: ("Slowest Growing".to_owned(), by_speed_rev).into(),
            oldest: ("Oldest".to_owned(), by_age).into(),
            youngest: ("Youngest".to_owned(), by_age_rev).into(),
        })
    }
}

impl PageComponent for HallOfFameItem {
    fn render(&self, _: &str) -> HtmlElement {
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

impl From<(String, Vec<(f32, &Plant)>)> for HallOfFameItem {
    fn from((s, plants): (String, Vec<(f32, &Plant)>)) -> HallOfFameItem {
        HallOfFameItem {
            title: s,
            plants: plants
                .iter()
                .take(10)
                .map(|(val, plant)| (val.to_string(), plant.name.clone()))
                .collect(),
        }
    }
}
