use super::{
    super::{
        super::html_components::{attribute::Attribute, component::HtmlComponent, div::Div},
        page::PageComponent,
    },
    plant_activity_table::PlantActivityTable,
    plant_growth_table::PlantGrowthTable,
};
use std::rc::Rc;

pub struct PlantActivities {
    watering_table: PlantActivityTable,
    fertilizing_table: PlantActivityTable,
    activity_table: PlantActivityTable,
    growth_table: PlantGrowthTable,
}

impl PageComponent for PlantActivities {
    fn render(&self) -> HtmlComponent {
        Div {
            attributes: vec![Attribute::Id("plant_activities_container".to_owned())],
            content: Rc::new(
                vec![
                    self.watering_table.render(),
                    self.fertilizing_table.render(),
                    self.activity_table.render(),
                    self.growth_table.render(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
