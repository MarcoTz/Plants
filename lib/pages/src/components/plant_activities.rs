use super::{
    page_component::PageComponent, plant_activity_table::PlantActivityTable,
    plant_growth_table::PlantGrowthTable,
};
use html::{attribute::Attribute, div::Div, html_element::HtmlElement};
use plants::{growth_item::GrowthItem, log_item::LogItem};
use std::rc::Rc;

pub struct PlantActivities {
    watering_table: PlantActivityTable,
    fertilizing_table: PlantActivityTable,
    activity_table: PlantActivityTable,
    growth_table: PlantGrowthTable,
}

impl PageComponent for PlantActivities {
    fn render(&self, date_format: &str) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("plant_activities_container".to_owned())],
            content: Rc::new(
                vec![
                    self.watering_table.render(date_format),
                    self.fertilizing_table.render(date_format),
                    self.activity_table.render(date_format),
                    self.growth_table.render(date_format),
                ]
                .into(),
            ),
        }
        .into()
    }
}

impl From<(&[LogItem], &[GrowthItem])> for PlantActivities {
    fn from((logs, growth): (&[LogItem], &[GrowthItem])) -> PlantActivities {
        let watering_activities: Vec<&LogItem> = logs
            .iter()
            .filter(|log| log.activity.to_lowercase().trim() == "watering")
            .collect();
        let fertilizing_activities: Vec<&LogItem> = logs
            .iter()
            .filter(|log| log.activity.to_lowercase().trim() == "fertilizing")
            .collect();
        let other_activities: Vec<&LogItem> = logs
            .iter()
            .filter(|x| !(watering_activities.contains(x) || fertilizing_activities.contains(x)))
            .collect();
        PlantActivities {
            watering_table: (watering_activities.as_slice(), false, false).into(),
            fertilizing_table: (fertilizing_activities.as_slice(), false, false).into(),
            activity_table: (other_activities.as_slice(), true, false).into(),
            growth_table: growth.into(),
        }
    }
}
