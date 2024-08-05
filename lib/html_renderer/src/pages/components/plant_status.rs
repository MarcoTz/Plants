use super::super::{
    super::html_components::{attribute::Attribute, component::HtmlComponent, div::Div},
    page::PageComponent,
};
use std::rc::Rc;

struct StatusItem {
    name: String,
    content: String,
}

pub struct PlantStatus {
    health: StatusItem,
    next_watering: StatusItem,
    next_fertilizing: StatusItem,
    last_watering: StatusItem,
    last_fertilizing: StatusItem,
    watering_frequency: StatusItem,
    fertilizing_frequency: StatusItem,
    current_height: StatusItem,
    current_width: StatusItem,
    growth_speed: StatusItem,
    is_autowatered: StatusItem,
    current_location: StatusItem,
    origin: StatusItem,
    age: StatusItem,
    notes: StatusItem,
}

impl PageComponent for PlantStatus {
    fn render(&self) -> HtmlComponent {
        let status_items = vec![
            self.health.render(),
            self.next_watering.render(),
            self.next_fertilizing.render(),
            self.last_watering.render(),
            self.last_fertilizing.render(),
            self.watering_frequency.render(),
            self.fertilizing_frequency.render(),
            self.current_height.render(),
            self.current_width.render(),
            self.growth_speed.render(),
            self.is_autowatered.render(),
            self.current_location.render(),
            self.origin.render(),
            self.age.render(),
            self.notes.render(),
        ];
        Div {
            attributes: vec![Attribute::Id("plant_status".to_owned())],
            content: Rc::new(status_items.into()),
        }
        .into()
    }
}

impl PageComponent for StatusItem {
    fn render(&self) -> HtmlComponent {
        Div {
            attributes: vec![Attribute::Class("status_item".to_owned())],
            content: {
                let name_str = self.name.clone();
                let content_str = self.content.clone();
                let div_content = vec![name_str.into(), HtmlComponent::Br, content_str.into()];
                Rc::new(div_content.into())
            },
        }
        .into()
    }
}
