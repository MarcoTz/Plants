use super::page_component::PageComponent;
use html::{attribute::Attribute, div::Div, html_element::HtmlElement};
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
    fn render(&self, date_format: &str) -> HtmlElement {
        let status_items = vec![
            self.health.render(date_format),
            self.next_watering.render(date_format),
            self.next_fertilizing.render(date_format),
            self.last_watering.render(date_format),
            self.last_fertilizing.render(date_format),
            self.watering_frequency.render(date_format),
            self.fertilizing_frequency.render(date_format),
            self.current_height.render(date_format),
            self.current_width.render(date_format),
            self.growth_speed.render(date_format),
            self.is_autowatered.render(date_format),
            self.current_location.render(date_format),
            self.origin.render(date_format),
            self.age.render(date_format),
            self.notes.render(date_format),
        ];
        Div {
            attributes: vec![Attribute::Id("plant_status".to_owned())],
            content: Rc::new(status_items.into()),
        }
        .into()
    }
}

impl PageComponent for StatusItem {
    fn render(&self, _: &str) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Class("status_item".to_owned())],
            content: {
                let name_str = self.name.clone();
                let content_str = self.content.clone();
                let div_content = vec![name_str.into(), HtmlElement::Br, content_str.into()];
                Rc::new(div_content.into())
            },
        }
        .into()
    }
}
