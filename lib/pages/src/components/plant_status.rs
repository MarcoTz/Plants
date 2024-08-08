use super::{super::errors::Error, page_component::PageComponent};
use chrono::NaiveDate;
use html::{attribute::Attribute, div::Div, html_element::HtmlElement};
use plants::plant::Plant;
use std::rc::Rc;

#[derive(Clone)]
struct StatusItem<T> {
    name: String,
    content: T,
}

#[derive(Clone)]
struct NaiveDateNoStr {
    date: NaiveDate,
}

impl From<NaiveDate> for NaiveDateNoStr {
    fn from(date: NaiveDate) -> NaiveDateNoStr {
        NaiveDateNoStr { date }
    }
}

pub struct PlantStatus {
    health: StatusItem<i32>,
    next_watering: Option<StatusItem<NaiveDateNoStr>>,
    next_fertilizing: Option<StatusItem<NaiveDateNoStr>>,
    last_watering: Option<StatusItem<NaiveDateNoStr>>,
    last_fertilizing: Option<StatusItem<NaiveDateNoStr>>,
    watering_frequency: Option<StatusItem<f32>>,
    fertilizing_frequency: Option<StatusItem<f32>>,
    current_height: StatusItem<f32>,
    current_width: StatusItem<f32>,
    growth_speed: StatusItem<f32>,
    is_autowatered: StatusItem<bool>,
    current_location: StatusItem<String>,
    origin: StatusItem<String>,
    age: StatusItem<i64>,
    notes: StatusItem<String>,
}

impl PageComponent for PlantStatus {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut status_items = vec![self.health.render(date_format)];
        match self.next_watering.clone() {
            None => (),
            Some(watering) => status_items.push(watering.render(date_format)),
        }

        match self.next_fertilizing.clone() {
            None => (),
            Some(fertilizing) => status_items.push(fertilizing.render(date_format)),
        }

        match self.last_watering.clone() {
            None => (),
            Some(watering) => status_items.push(watering.render(date_format)),
        }

        match self.last_fertilizing.clone() {
            None => (),
            Some(fertilizing) => status_items.push(fertilizing.render(date_format)),
        }

        match self.watering_frequency.clone() {
            None => (),
            Some(fertilizing) => status_items.push(fertilizing.render(date_format)),
        }

        match self.fertilizing_frequency.clone() {
            None => (),
            Some(fertilizing) => status_items.push(fertilizing.render(date_format)),
        }
        status_items.extend(vec![
            self.current_height.render(date_format),
            self.current_width.render(date_format),
            self.growth_speed.render(date_format),
            self.is_autowatered.render(date_format),
            self.current_location.render(date_format),
            self.origin.render(date_format),
            self.age.render(date_format),
            self.notes.render(date_format),
        ]);
        Div {
            attributes: vec![Attribute::Id("plant_status".to_owned())],
            content: Rc::new(status_items.into()),
        }
        .into()
    }
}

impl<T: ToString> PageComponent for StatusItem<T> {
    fn render(&self, _: &str) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Class("status_item".to_owned())],
            content: {
                let name_str = self.name.clone();
                let content_str = self.content.to_string();
                let div_content = vec![name_str.into(), HtmlElement::Br, content_str.into()];
                Rc::new(div_content.into())
            },
        }
        .into()
    }
}

impl PageComponent for StatusItem<NaiveDateNoStr> {
    fn render(&self, date_format: &str) -> HtmlElement {
        StatusItem {
            name: self.name.clone(),
            content: self.content.date.format(date_format).to_string(),
        }
        .render(date_format)
    }
}

impl<T, U: Into<T>> From<(&str, U)> for StatusItem<T> {
    fn from((name, u): (&str, U)) -> StatusItem<T> {
        StatusItem {
            name: name.to_owned(),
            content: u.into(),
        }
    }
}
impl TryFrom<&Plant> for PlantStatus {
    type Error = Error;
    fn try_from(plant: &Plant) -> Result<PlantStatus, Self::Error> {
        let health = plant.get_health()?;
        let next_watering = plant.get_next_watering();
        let next_fertilizing = plant.get_next_fertilizing();
        let last_watering = plant.get_last_watering();
        let last_fertilizing = plant.get_last_fertilizing();
        let watering_frequency = plant.get_watering_frequency();
        let fertilizing_frequency = plant.get_fertilizing_frequency();
        let current_height = plant.get_height()?;
        let current_width = plant.get_width()?;
        let growth_speed = plant.get_growth_speed()?;
        let age = plant.get_age_days();
        let notes_str = plant.notes.join(", ");

        Ok(PlantStatus {
            health: ("Health", health).into(),
            next_watering: next_watering.map(|wt| ("Next Watering", wt).into()),
            next_fertilizing: next_fertilizing.map(|ft| ("Next Fertilizing", ft).into()),
            last_watering: last_watering.map(|wt| ("Last Watering", wt.date).into()),
            last_fertilizing: last_fertilizing.map(|ft| ("Last Fertilizing", ft.date).into()),
            watering_frequency: watering_frequency.map(|frq| ("Watering Frequency", frq).into()),
            fertilizing_frequency: fertilizing_frequency
                .map(|frq| ("Fertilizing Frequency", frq).into()),
            current_height: ("Current Height", current_height).into(),
            current_width: ("Current Width", current_width).into(),
            growth_speed: ("Growth Speed", growth_speed).into(),
            is_autowatered: ("Autowatered", plant.auto_water).into(),
            current_location: ("Current Location", plant.location.clone()).into(),
            origin: ("Origin", plant.origin.clone()).into(),
            age: ("Age (Days)", age).into(),
            notes: ("Notes", notes_str).into(),
        })
    }
}
