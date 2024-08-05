use super::super::{
    super::html::{attribute::Attribute, div::Div, html_element::HtmlElement, link::Link},
    page::PageComponent,
};
use chrono::{Datelike, NaiveDate};
use std::rc::Rc;

pub struct NextActivity {
    next_activities: Vec<NextActivityItem>,
}

pub struct NextActivityItem {
    date: NaiveDate,
    date_format: String,
    activity: String,
    plants: Vec<(String, String)>,
}

impl PageComponent for NextActivity {
    fn render(&self) -> HtmlElement {
        let mut next_activities_comp = vec![];
        for next_activity in self.next_activities.iter() {
            next_activities_comp.push(next_activity.render());
        }

        Div {
            attributes: vec![Attribute::Id("upcoming_tasks_container".to_owned())],
            content: Rc::new(next_activities_comp.into()),
        }
        .into()
    }
}

impl PageComponent for NextActivityItem {
    fn render(&self) -> HtmlElement {
        let mut div_content = vec![];

        let mut date_str = self.date.weekday().to_string();
        date_str.push_str(&self.date.format(&self.date_format).to_string());
        div_content.push(date_str.into());
        div_content.push(HtmlElement::Br);

        let activity_header = Div {
            attributes: vec![Attribute::Class("activity_header".to_owned())],
            content: Rc::new(self.activity.clone().into()),
        }
        .into();
        div_content.push(activity_header);

        for (plant_name, plant_url) in self.plants.iter() {
            div_content.push(
                Link {
                    attributes: vec![Attribute::Href(plant_url.clone())],
                    content: Rc::new(plant_name.clone().into()),
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
