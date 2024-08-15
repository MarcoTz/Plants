use crate::{page::PageComponent, shared::plant_link::PlantLink};
use chrono::{Datelike, NaiveDate};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement},
};
use plants::plant::Plant;
use std::collections::HashMap;
use std::rc::Rc;

pub struct UpcomingTasks {
    tasks: Vec<TaskBlock>,
}

#[derive(Clone)]
pub struct TaskBlock {
    date: NaiveDate,
    items: Vec<TaskItem>,
}
#[derive(Clone, Debug)]
pub struct TaskItem {
    activity: String,
    plants: Vec<PlantLink>,
}

impl PageComponent for TaskItem {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut task_content = vec![self.activity.clone().into(), HtmlElement::Br];

        for (i, plant_link) in self.plants.iter().enumerate() {
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
            if plant.auto_water {
                continue;
            }

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
