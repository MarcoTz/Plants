use crate::{page::PageComponent, shared::plant_link::PlantLink};
use chrono::{Datelike, Local, NaiveDate, TimeDelta};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement, Table, Td, Tr},
};
use plants::plant::Plant;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct UpcomingTasks {
    tasks: Vec<TaskBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskBlock {
    date: NaiveDate,
    items: Vec<TaskItem>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskItem {
    plant: PlantLink,
    watering: bool,
    fertilizing: bool,
    growth: bool,
}

impl PageComponent for TaskItem {
    fn render(&self, date_format: &str) -> HtmlElement {
        let get_sym = |b: bool| {
            if b {
                "✅".to_owned()
            } else {
                "❌".to_owned()
            }
        };
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new(self.plant.render(date_format)),
                },
                Td {
                    content: Rc::new(get_sym(self.watering).into()),
                },
                Td {
                    content: Rc::new(get_sym(self.fertilizing).into()),
                },
                Td {
                    content: Rc::new(get_sym(self.growth).into()),
                },
            ],
        }
        .into()
    }
}
impl PageComponent for TaskBlock {
    fn render(&self, date_format: &str) -> HtmlElement {
        let header_str =
            self.date.weekday().to_string() + ", " + &self.date.format(date_format).to_string();
        let mut rows = vec![Tr {
            attributes: vec![Attribute::Class(vec!["header_row".to_owned()])],
            cols: vec![
                Td {
                    content: Rc::new("Plant".to_owned().into()),
                },
                Td {
                    content: Rc::new("🌊".to_owned().into()),
                },
                Td {
                    content: Rc::new("💩".to_owned().into()),
                },
                Td {
                    content: Rc::new("📏".to_owned().into()),
                },
            ],
        }
        .into()];

        for item in &self.items {
            rows.push(item.render(date_format));
        }

        Div {
            attributes: vec![Attribute::Class(vec!["task_block".to_owned()])],
            content: Rc::new(
                vec![
                    header_str.into(),
                    Table {
                        attributes: vec![],
                        rows,
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
impl PageComponent for UpcomingTasks {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut tasks = vec![];

        let mut tasks_sorted = self.tasks.clone();
        tasks_sorted.sort_by(|block1, block2| block1.date.cmp(&block2.date));

        for task in tasks_sorted.iter() {
            tasks.push(task.render(date_format));
        }

        vec![
            Headline {
                attributes: vec![],
                size: HeaderSize::H1,
                content: Rc::new("Upcoming Tasks".to_owned().into()),
            }
            .into(),
            Div {
                attributes: vec![
                    Attribute::Id("tasks_container".to_owned()),
                    Attribute::Class(vec![
                        "flex_container".to_owned(),
                        "alternating_children".to_owned(),
                    ]),
                ],
                content: Rc::new(tasks.into()),
            }
            .into(),
        ]
        .into()
    }
}

impl From<&[Plant]> for UpcomingTasks {
    fn from(plants: &[Plant]) -> UpcomingTasks {
        struct PlantWDates<'a> {
            plant: &'a Plant,
            watering: Option<NaiveDate>,
            fertilizing: Option<NaiveDate>,
            growth: NaiveDate,
        }

        let plants_with_dates: Vec<PlantWDates> = plants
            .iter()
            .map(|plant| PlantWDates {
                plant,
                watering: plant.get_next_watering(),
                fertilizing: plant.get_next_fertilizing(),
                growth: plant.get_next_growth(),
            })
            .collect();

        let max_date = plants_with_dates
            .iter()
            .map(|pl| {
                pl.watering
                    .max(pl.fertilizing)
                    .max(Some(pl.growth))
                    .unwrap()
            })
            .max()
            .unwrap_or(Local::now().date_naive());

        let mut last_date = Local::now().date_naive() - TimeDelta::days(1);
        let diff = (max_date - last_date).num_days();

        let mut tasks = vec![];
        for _ in 0..=diff {
            let next_date = last_date + TimeDelta::days(1);

            let next_plants: Vec<&PlantWDates> = plants_with_dates
                .iter()
                .filter(|pl| {
                    pl.watering == Some(next_date)
                        || pl.fertilizing == Some(next_date)
                        || pl.growth == next_date
                })
                .collect();
            if next_plants.is_empty() {
                last_date = next_date;
                continue;
            }

            let mut next_items: Vec<TaskItem> = next_plants
                .iter()
                .map(|pl| TaskItem {
                    plant: PlantLink::from((pl.plant, "plants")),
                    watering: pl.watering == Some(next_date),
                    fertilizing: pl.fertilizing == Some(next_date),
                    growth: pl.growth == next_date,
                })
                .collect();
            let sort_key = |pl: &TaskItem| {
                (if pl.watering { 4 } else { 0 })
                    + (if pl.fertilizing { 3 } else { 0 })
                    + (if pl.growth { 2 } else { 0 })
            };
            next_items.sort_by(|pl1, pl2| sort_key(pl2).cmp(&sort_key(pl1)));

            tasks.push(TaskBlock {
                date: next_date,
                items: next_items,
            });

            last_date = next_date;
        }

        UpcomingTasks { tasks }
    }
}
