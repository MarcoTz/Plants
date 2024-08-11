use super::page_component::PageComponent;
use chrono::{Datelike, NaiveDate};
use html::{
    a::A,
    attribute::Attribute,
    div::Div,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
};
use plants::plant::Plant;
use std::collections::HashMap;
use std::rc::Rc;

pub struct NextActivity {
    next_activities: Vec<NextActivityBlock>,
}

#[derive(Clone, Debug)]
struct PlantLink {
    plant_name: String,
    plant_url: String,
}
pub struct NextActivityBlock {
    date: NaiveDate,
    items: Vec<NextActivityItem>,
}
#[derive(Clone, Debug)]
pub struct NextActivityItem {
    activity: String,
    plants: Vec<PlantLink>,
}

impl PageComponent for NextActivity {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut next_activities_comp = vec![];
        for next_activity in self.next_activities.iter() {
            next_activities_comp.push(next_activity.render(date_format));
        }
        vec![
            Headline {
                size: HeaderSize::H1,
                content: Rc::new("Upcoming Tasks".to_owned().into()),
            }
            .into(),
            Div {
                attributes: vec![Attribute::Id("upcoming_tasks_container".to_owned())],
                content: Rc::new(next_activities_comp.into()),
            }
            .into(),
        ]
        .into()
    }
}
impl PageComponent for NextActivityItem {
    fn render(&self, _: &str) -> HtmlElement {
        let mut header_content = vec![self.activity.clone().into(), HtmlElement::Br];
        for plant_link in self.plants.iter() {
            header_content.push(
                A {
                    attributes: vec![Attribute::Href(plant_link.plant_url.clone())],
                    content: Rc::new(plant_link.plant_name.clone().into()),
                }
                .into(),
            );
            header_content.push(", ".to_owned().into())
        }

        Div {
            attributes: vec![Attribute::Class("activity_header".to_owned())],
            content: Rc::new(header_content.into()),
        }
        .into()
    }
}
impl PageComponent for NextActivityBlock {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut div_content = vec![];

        let mut date_str = self.date.weekday().to_string();
        date_str.push_str(" ,");
        date_str.push_str(&self.date.format(date_format).to_string());
        div_content.push(date_str.into());
        div_content.push(HtmlElement::Br);

        for activity_item in self.items.iter() {
            div_content.push(activity_item.render(date_format));
        }

        Div {
            attributes: vec![Attribute::Class("next_activity_item".to_owned())],
            content: Rc::new(div_content.into()),
        }
        .into()
    }
}

impl From<&[Plant]> for NextActivity {
    fn from(plants: &[Plant]) -> NextActivity {
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
            let plant_tuple = PlantLink {
                plant_url: plant.get_url("plants/"),
                plant_name: plant.name.clone(),
            };
            map_update(
                plant.get_next_growth(),
                plant_tuple.clone(),
                &mut next_growth_dates,
            );
            match (next_watering, next_fertilizing) {
                (None, None) => (),
                (Some(watering_date), None) => {
                    map_update(watering_date, plant_tuple, &mut next_watering_dates)
                }
                (None, Some(fertilizing_date)) => {
                    map_update(fertilizing_date, plant_tuple, &mut next_fertilizing_dates)
                }
                (Some(watering_date), Some(fertilizing_date)) => {
                    if watering_date == fertilizing_date {
                        map_update(watering_date, plant_tuple, &mut next_both_dates)
                    } else {
                        map_update(watering_date, plant_tuple.clone(), &mut next_watering_dates);
                        map_update(fertilizing_date, plant_tuple, &mut next_fertilizing_dates);
                    }
                }
            }
        }

        let mut next_activities: HashMap<NaiveDate, Vec<NextActivityItem>> = HashMap::new();
        let plant_update =
            |activity_date: &NaiveDate,
             activity_name: &str,
             activity_plants: &[PlantLink],
             activity_map: &mut HashMap<NaiveDate, Vec<NextActivityItem>>| {
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

        NextActivity {
            next_activities: next_activities
                .iter()
                .map(|(date, items)| (date, items.as_slice()).into())
                .collect(),
        }
    }
}

impl From<(&NaiveDate, &[NextActivityItem])> for NextActivityBlock {
    fn from((date, items): (&NaiveDate, &[NextActivityItem])) -> NextActivityBlock {
        NextActivityBlock {
            date: *date,
            items: items.iter().cloned().collect(),
        }
    }
}

impl From<(&str, &[PlantLink])> for NextActivityItem {
    fn from((activity_str, plants): (&str, &[PlantLink])) -> NextActivityItem {
        NextActivityItem {
            activity: activity_str.to_owned(),
            plants: plants.to_vec(),
        }
    }
}
