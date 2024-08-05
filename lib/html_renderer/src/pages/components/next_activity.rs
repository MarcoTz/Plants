use super::super::{
    super::html_components::{component::HtmlComponent, div::Div, link::Link},
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
    fn render(&self) -> HtmlComponent {
        let mut next_activities_comp = vec![];
        for next_activity in self.next_activities.iter() {
            next_activities_comp.push(next_activity.render());
        }

        Div {
            id: Some("upcoming_tasks.container".to_owned()),
            class: None,
            contents: Rc::new(next_activities_comp.into()),
        }
        .into()
    }
}

impl PageComponent for NextActivityItem {
    fn render(&self) -> HtmlComponent {
        let mut div_contents = vec![];

        let mut date_str = self.date.weekday().to_string();
        date_str.push_str(&self.date.format(&self.date_format).to_string());
        div_contents.push(date_str.into());
        div_contents.push(HtmlComponent::Br);

        let activity_header = Div {
            id: None,
            class: Some("activity_header".to_owned()),
            contents: Rc::new(self.activity.clone().into()),
        }
        .into();
        div_contents.push(activity_header);

        for (plant_name, plant_url) in self.plants.iter() {
            div_contents.push(
                Link {
                    href: plant_url.clone(),
                    contents: Rc::new(plant_name.clone().into()),
                }
                .into(),
            );
            div_contents.push(", ".to_owned().into())
        }

        Div {
            class: Some("next_activity_item".to_owned()),
            id: None,
            contents: Rc::new(div_contents.into()),
        }
        .into()
    }
}
