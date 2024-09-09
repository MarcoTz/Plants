use crate::{page::PageComponent, shared::plant_link::PlantLink};
use chrono::{Datelike, NaiveDate};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement},
};
use plants::plant::Plant;
use std::collections::HashMap;
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
    activity: String,
    plants: Vec<PlantLink>,
}

impl PageComponent for TaskItem {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut task_content = vec![self.activity.clone().into(), HtmlElement::Br];
        let mut plants_sorted = self.plants.clone();
        plants_sorted.sort_by(|item1, item2| item1.plant_name.cmp(&item2.plant_name));

        for (i, plant_link) in plants_sorted.iter().enumerate() {
            task_content.push(plant_link.render(date_format));
            if i < self.plants.len() {
                task_content.push(", ".to_owned().into());
            }
        }

        Div {
            attributes: vec![Attribute::Class(vec!["upcoming_task".to_owned()])],
            content: Rc::new(task_content.into()),
        }
        .into()
    }
}
impl PageComponent for TaskBlock {
    fn render(&self, date_format: &str) -> HtmlElement {
        let header_str =
            self.date.weekday().to_string() + ", " + &self.date.format(date_format).to_string();
        let mut div_content = vec![header_str.into(), HtmlElement::Br];

        for activity_item in self.items.iter() {
            div_content.push(activity_item.render(date_format));
        }

        Div {
            attributes: vec![Attribute::Class(vec!["task_block".to_owned()])],
            content: Rc::new(div_content.into()),
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
        log::info!("Loading Upcoming Tasks");
        let mut next_watering_dates: HashMap<NaiveDate, Vec<PlantLink>> = HashMap::new();
        let mut next_fertilizing_dates: HashMap<NaiveDate, Vec<PlantLink>> = HashMap::new();
        let mut next_both_dates: HashMap<NaiveDate, Vec<PlantLink>> = HashMap::new();
        let mut next_growth_dates: HashMap<NaiveDate, Vec<PlantLink>> = HashMap::new();

        let map_update =
            |key: NaiveDate, value: PlantLink, map: &mut HashMap<NaiveDate, Vec<PlantLink>>| {
                match map.get_mut(&key) {
                    None => {
                        map.insert(key, vec![value]);
                    }
                    Some(tup_vec) => tup_vec.push(value),
                }
            };
        for plant in plants.iter() {
            let next_watering = plant.get_next_watering();
            let next_fertilizing = plant.get_next_fertilizing();
            let plant_link: PlantLink = (plant, "plants").into();
            map_update(
                plant.get_next_growth(),
                plant_link.clone(),
                &mut next_growth_dates,
            );
            match (next_watering, next_fertilizing) {
                (None, None) => (),
                (Some(watering_date), None) => {
                    map_update(watering_date, plant_link, &mut next_watering_dates)
                }
                (None, Some(fertilizing_date)) => {
                    map_update(fertilizing_date, plant_link, &mut next_fertilizing_dates)
                }
                (Some(watering_date), Some(fertilizing_date)) => {
                    if watering_date == fertilizing_date {
                        map_update(watering_date, plant_link, &mut next_both_dates)
                    } else {
                        map_update(watering_date, plant_link.clone(), &mut next_watering_dates);
                        map_update(fertilizing_date, plant_link, &mut next_fertilizing_dates);
                    }
                }
            }
        }

        let mut next_activities: HashMap<NaiveDate, Vec<TaskItem>> = HashMap::new();
        let plant_update =
            |activity_date: &NaiveDate,
             activity_name: &str,
             activity_plants: &[PlantLink],
             activity_map: &mut HashMap<NaiveDate, Vec<TaskItem>>| {
                let plants_item = (activity_name, activity_plants).into();
                match activity_map.get_mut(activity_date) {
                    None => {
                        activity_map.insert(*activity_date, vec![plants_item]);
                    }
                    Some(activity_vec) => {
                        activity_vec.push(plants_item);
                    }
                }
            };
        for (watering_date, watering_plants) in next_watering_dates.iter() {
            plant_update(
                watering_date,
                "🌊 Watering 🌊",
                watering_plants,
                &mut next_activities,
            )
        }

        for (fertilizing_date, fertilizing_plants) in next_fertilizing_dates.iter() {
            plant_update(
                fertilizing_date,
                "💩 Fertilizing 💩",
                fertilizing_plants,
                &mut next_activities,
            )
        }

        for (both_date, both_plants) in next_both_dates.iter() {
            plant_update(
                both_date,
                "🌊 Watering+Fertilizing 💩",
                both_plants,
                &mut next_activities,
            )
        }

        for (growth_date, growth_plants) in next_growth_dates.iter() {
            plant_update(
                growth_date,
                "📏 Growth 📏",
                growth_plants,
                &mut next_activities,
            )
        }

        UpcomingTasks {
            tasks: next_activities
                .iter()
                .map(|(date, items)| (date, items.as_slice()).into())
                .collect(),
        }
    }
}

impl From<(&NaiveDate, &[TaskItem])> for TaskBlock {
    fn from((date, items): (&NaiveDate, &[TaskItem])) -> TaskBlock {
        TaskBlock {
            date: *date,
            items: items.to_vec(),
        }
    }
}

impl From<(&str, &[PlantLink])> for TaskItem {
    fn from((activity_str, plants): (&str, &[PlantLink])) -> TaskItem {
        TaskItem {
            activity: activity_str.to_owned(),
            plants: plants.to_vec(),
        }
    }
}

#[cfg(test)]
mod upcoming_tasks_tests {
    use super::{PageComponent, PlantLink, TaskBlock, TaskItem, UpcomingTasks};
    use crate::test_common::{
        example_plant1, example_plant2, example_plant3, example_plantlink1, example_plantlink2,
        example_plantlink3, example_species, DATE_FORMAT,
    };
    use chrono::{Datelike, Local};
    use html::{
        attribute::Attribute,
        elements::{Div, HeaderSize, Headline, HtmlElement, A},
    };
    use plants::plant::PlantSpecies;
    use std::rc::Rc;

    fn example_upcoming_tasks() -> UpcomingTasks {
        UpcomingTasks {
            tasks: vec![TaskBlock {
                date: Local::now().date_naive(),
                items: vec![
                    TaskItem {
                        activity: "🌊 Watering 🌊".to_owned(),
                        plants: vec![example_plantlink1()],
                    },
                    TaskItem {
                        activity: "💩 Fertilizing 💩".to_owned(),
                        plants: vec![example_plantlink2()],
                    },
                    TaskItem {
                        activity: "🌊 Watering+Fertilizing 💩".to_owned(),
                        plants: vec![example_plantlink3()],
                    },
                    TaskItem {
                        activity: "📏 Growth 📏".to_owned(),
                        plants: vec![
                            example_plantlink1(),
                            example_plantlink2(),
                            example_plantlink3(),
                        ],
                    },
                ],
            }],
        }
    }

    fn example_task_block() -> TaskBlock {
        TaskBlock {
            date: Local::now().date_naive(),
            items: vec![example_task_item()],
        }
    }

    fn example_task_item() -> TaskItem {
        TaskItem {
            activity: "📏 Growth 📏".to_string(),
            plants: vec![PlantLink::from((&example_plant3(), "plants"))],
        }
    }

    #[test]
    fn render_task_item() {
        let result = example_task_item().render(DATE_FORMAT);
        let expected = Div {
            attributes: vec![Attribute::Class(vec!["upcoming_task".to_owned()])],
            content: Rc::new(
                vec![
                    "📏 Growth 📏".to_owned().into(),
                    HtmlElement::Br,
                    A {
                        attributes: vec![
                            Attribute::Href("plants/Plant3.html".to_owned()),
                            Attribute::Class(vec!["plant_link".to_owned()]),
                        ],
                        content: Rc::new("Plant3".to_owned().into()),
                    }
                    .into(),
                    ", ".to_owned().into(),
                ]
                .into(),
            ),
        }
        .into();

        assert_eq!(result, expected)
    }

    #[test]
    fn render_task_block() {
        let result = example_task_block().render(DATE_FORMAT);
        let today = Local::now().date_naive();

        let expected = Div {
            attributes: vec![Attribute::Class(vec!["task_block".to_owned()])],
            content: Rc::new(
                vec![
                    (today.weekday().to_string() + ", " + &today.format(DATE_FORMAT).to_string())
                        .into(),
                    HtmlElement::Br,
                    Div {
                        attributes: vec![Attribute::Class(vec!["upcoming_task".to_owned()])],
                        content: Rc::new(
                            vec![
                                "📏 Growth 📏".to_owned().into(),
                                HtmlElement::Br,
                                A {
                                    attributes: vec![
                                        Attribute::Href("plants/Plant3.html".to_owned()),
                                        Attribute::Class(vec!["plant_link".to_owned()]),
                                    ],
                                    content: Rc::new("Plant3".to_owned().into()),
                                }
                                .into(),
                                ", ".to_owned().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into();

        assert_eq!(result, expected)
    }

    #[test]
    fn render_upcoming_tasks() {
        let result = example_upcoming_tasks().render(DATE_FORMAT);
        let today = Local::now().date_naive();
        let expected = vec![
            Headline {
                size: HeaderSize::H1,
                attributes: vec![],
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
                content: Rc::new(
                    vec![Div {
                        attributes: vec![Attribute::Class(vec!["task_block".to_owned()])],
                        content: Rc::new(
                            vec![
                                (today.weekday().to_string()
                                    + ", "
                                    + &today.format(DATE_FORMAT).to_string())
                                    .into(),
                                HtmlElement::Br,
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "upcoming_task".to_owned()
                                    ])],
                                    content: Rc::new(
                                        vec![
                                            "🌊 Watering 🌊".to_owned().into(),
                                            HtmlElement::Br,
                                            A {
                                                attributes: vec![
                                                    Attribute::Href(
                                                        "plants/Plant1.html".to_owned(),
                                                    ),
                                                    Attribute::Class(vec!["plant_link".to_owned()]),
                                                ],
                                                content: Rc::new("Plant1".to_owned().into()),
                                            }
                                            .into(),
                                            ", ".to_owned().into(),
                                        ]
                                        .into(),
                                    ),
                                }
                                .into(),
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "upcoming_task".to_owned()
                                    ])],
                                    content: Rc::new(
                                        vec![
                                            "💩 Fertilizing 💩".to_owned().into(),
                                            HtmlElement::Br,
                                            A {
                                                attributes: vec![
                                                    Attribute::Href(
                                                        "plants/Plant2.html".to_owned(),
                                                    ),
                                                    Attribute::Class(vec!["plant_link".to_owned()]),
                                                ],
                                                content: Rc::new("Plant2".to_owned().into()),
                                            }
                                            .into(),
                                            ", ".to_owned().into(),
                                        ]
                                        .into(),
                                    ),
                                }
                                .into(),
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "upcoming_task".to_owned()
                                    ])],
                                    content: Rc::new(
                                        vec![
                                            "🌊 Watering+Fertilizing 💩".to_owned().into(),
                                            HtmlElement::Br,
                                            A {
                                                attributes: vec![
                                                    Attribute::Href(
                                                        "plants/Plant3.html".to_owned(),
                                                    ),
                                                    Attribute::Class(vec!["plant_link".to_owned()]),
                                                ],
                                                content: Rc::new("Plant3".to_owned().into()),
                                            }
                                            .into(),
                                            ", ".to_owned().into(),
                                        ]
                                        .into(),
                                    ),
                                }
                                .into(),
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "upcoming_task".to_owned()
                                    ])],
                                    content: Rc::new(
                                        vec![
                                            "📏 Growth 📏".to_owned().into(),
                                            HtmlElement::Br,
                                            A {
                                                attributes: vec![
                                                    Attribute::Href(
                                                        "plants/Plant1.html".to_owned(),
                                                    ),
                                                    Attribute::Class(vec!["plant_link".to_owned()]),
                                                ],
                                                content: Rc::new("Plant1".to_owned().into()),
                                            }
                                            .into(),
                                            ", ".to_owned().into(),
                                            A {
                                                attributes: vec![
                                                    Attribute::Href(
                                                        "plants/Plant2.html".to_owned(),
                                                    ),
                                                    Attribute::Class(vec!["plant_link".to_owned()]),
                                                ],
                                                content: Rc::new("Plant2".to_owned().into()),
                                            }
                                            .into(),
                                            ", ".to_owned().into(),
                                            A {
                                                attributes: vec![
                                                    Attribute::Href(
                                                        "plants/Plant3.html".to_owned(),
                                                    ),
                                                    Attribute::Class(vec!["plant_link".to_owned()]),
                                                ],
                                                content: Rc::new("Plant3".to_owned().into()),
                                            }
                                            .into(),
                                            ", ".to_owned().into(),
                                        ]
                                        .into(),
                                    ),
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into()]
                    .into(),
                ),
            }
            .into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn item_into() {
        let result = TaskItem::from((
            "📏 Growth 📏",
            vec![PlantLink::from((&example_plant3(), "plants"))].as_slice(),
        ));
        let expected = example_task_item();
        assert_eq!(result, expected)
    }

    #[test]
    fn block_into() {
        let result = TaskBlock::from((
            &Local::now().date_naive(),
            vec![example_task_item()].as_slice(),
        ));
        let expected = example_task_block();
        assert_eq!(result, expected)
    }

    #[test]
    fn tasks_into() {
        let mut watering_plant = example_plant1();
        let mut watering_species = example_species();
        watering_species.avg_fertilizing_days = None;
        watering_plant.info.species = PlantSpecies::Species(Box::new(watering_species));

        let result = UpcomingTasks::from(
            vec![watering_plant, example_plant2(), example_plant3()].as_slice(),
        );
        let expected = example_upcoming_tasks();
        assert_eq!(result, expected)
    }
}
