use super::{
    page_component::PageComponent, plant_activity_table::PlantActivityTable,
    plant_growth_table::PlantGrowthTable,
};
use html::{attribute::Attribute, div::Div, html_element::HtmlElement};
use std::rc::Rc;

pub struct PlantActivities {
    watering_table: PlantActivityTable,
    fertilizing_table: PlantActivityTable,
    activity_table: PlantActivityTable,
    growth_table: PlantGrowthTable,
}

impl PageComponent for PlantActivities {
    fn render(&self) -> HtmlElement {
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
