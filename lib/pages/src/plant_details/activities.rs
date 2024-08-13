use super::{activity_table::ActivityTable, growth_table::GrowthTable};
use crate::components::page_component::PageComponent;
use html::{
    attribute::Attribute,
    div::Div,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
};
use plants::{growth_item::GrowthItem, log_item::LogItem, plant::Plant};
use std::rc::Rc;

pub struct PlantActivities {
    watering_table: ActivityTable,
    fertilizing_table: ActivityTable,
    activity_table: ActivityTable,
    growth_table: GrowthTable,
}

impl PageComponent for PlantActivities {
    fn render(&self, date_format: &str) -> HtmlElement {
        Div {
            attributes: vec![
                Attribute::Id("plant_activities_container".to_owned()),
                Attribute::Class(vec!["flex_container".to_owned()]),
            ],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    attributes: vec![],
                                    size: HeaderSize::H2,
                                    content: Rc::new("Watering".to_owned().into()),
                                }
                                .into(),
                                self.watering_table.render(date_format),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    attributes: vec![],
                                    size: HeaderSize::H2,
                                    content: Rc::new("Fertilizing".to_owned().into()),
                                }
                                .into(),
                                self.fertilizing_table.render(date_format),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    attributes: vec![],
                                    size: HeaderSize::H2,
                                    content: Rc::new("Other Activities".to_owned().into()),
                                }
                                .into(),
                                self.activity_table.render(date_format),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_table".to_owned()])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    attributes: vec![],
                                    size: HeaderSize::H2,
                                    content: Rc::new("Growth".to_owned().into()),
                                }
                                .into(),
                                self.growth_table.render(date_format),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}

impl From<&Plant> for PlantActivities {
    fn from(plant: &Plant) -> PlantActivities {
        PlantActivities::from((plant.activities.as_slice(), plant.growth.as_slice()))
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
            watering_table: (watering_activities.as_slice(), false).into(),
            fertilizing_table: (fertilizing_activities.as_slice(), false).into(),
            activity_table: (other_activities.as_slice(), true).into(),
            growth_table: growth.into(),
        }
    }
}
