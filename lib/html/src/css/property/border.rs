use super::{Direction, Property};
use crate::render::Render;

#[derive(Clone)]
pub enum Border {
    Side(Direction),
    Color,
    Style,
    Radius,
    Collapse,
}

impl Render for Border {
    fn render(&self) -> String {
        match self {
            Border::Side(Direction::Top) => "border-top".to_owned(),
            Border::Side(Direction::Bottom) => "border-bottom".to_owned(),
            Border::Side(Direction::Left) => "border-left".to_owned(),
            Border::Side(Direction::Right) => "border-right".to_owned(),
            Border::Side(Direction::All) => "border".to_owned(),
            Border::Color => "border-color".to_owned(),
            Border::Style => "border-style".to_owned(),
            Border::Radius => "border-radius".to_owned(),
            Border::Collapse => "border-collapse".to_owned(),
        }
    }
}

impl From<Border> for Property {
    fn from(border: Border) -> Property {
        Property::Border(border)
    }
}
