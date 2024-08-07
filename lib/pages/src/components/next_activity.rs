use super::page_component::PageComponent;
use chrono::{Datelike, NaiveDate};
use html::{a::A, attribute::Attribute, div::Div, html_element::HtmlElement};
use plants::plant::Plant;
use std::collections::HashMap;
use std::rc::Rc;

pub struct NextActivity {
    next_activities: Vec<NextActivityItem>,
}

#[derive(Clone)]
struct PlantLink {
    plant_name: String,
    plant_url: String,
}
pub struct NextActivityItem {
    date: NaiveDate,
    activity: String,
    plants: Vec<PlantLink>,
}

impl PageComponent for NextActivity {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut next_activities_comp = vec![];
        for next_activity in self.next_activities.iter() {
            next_activities_comp.push(next_activity.render(date_format));
        }

        Div {
            attributes: vec![Attribute::Id("upcoming_tasks_container".to_owned())],
            content: Rc::new(next_activities_comp.into()),
        }
        .into()
    }
}

impl PageComponent for NextActivityItem {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut div_content = vec![];

        let mut date_str = self.date.weekday().to_string();
        date_str.push_str(&self.date.format(date_format).to_string());
        div_content.push(date_str.into());
        div_content.push(HtmlElement::Br);

        let activity_header = Div {
            attributes: vec![Attribute::Class("activity_header".to_owned())],
            content: Rc::new(self.activity.clone().into()),
        }
        .into();
        div_content.push(activity_header);

        for plant_link in self.plants.iter() {
            div_content.push(
                A {
                    attributes: vec![Attribute::Href(plant_link.plant_url.clone())],
                    content: Rc::new(plant_link.plant_name.clone().into()),
                }
                .into(),
            );
            div_content.push(", ".to_owned().into())
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

        let mut next_dates_vec: Vec<(&str, &NaiveDate, &Vec<PlantLink>)> = next_watering_dates
            .iter()
            .map(|(key, val)| ("Watering", key, val))
            .collect();
        next_dates_vec.extend(
            next_fertilizing_dates
                .iter()
                .map(|(key, val)| ("Fertilizing", key, val))
                .collect::<Vec<(&str, &NaiveDate, &Vec<PlantLink>)>>(),
        );
        next_dates_vec.extend(
            next_both_dates
                .iter()
                .map(|(key, val)| ("Watering+Fertilizing", key, val))
                .collect::<Vec<(&str, &NaiveDate, &Vec<PlantLink>)>>(),
        );
        next_dates_vec.extend(
            next_growth_dates
                .iter()
                .map(|(key, val)| ("Growth", key, val))
                .collect::<Vec<(&str, &NaiveDate, &Vec<PlantLink>)>>(),
        );

        let next_activities: Vec<NextActivityItem> = next_dates_vec
            .iter()
            .cloned()
            .map(|act| act.into())
            .collect();
        NextActivity { next_activities }
    }
}

impl From<(&str, &NaiveDate, &Vec<PlantLink>)> for NextActivityItem {
    fn from((activity_str, date, plants): (&str, &NaiveDate, &Vec<PlantLink>)) -> NextActivityItem {
        NextActivityItem {
            activity: activity_str.to_owned(),
            date: date.clone(),
            plants: plants.clone(),
        }
    }
}
