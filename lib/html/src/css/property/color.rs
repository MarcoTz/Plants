use super::Property;
use crate::render::Render;

#[derive(Clone)]
pub enum Color {
    Background,
    Color,
}

impl Render for Color {
    fn render(&self) -> String {
        match self {
            Color::Background => "background".to_owned(),
            Color::Color => "color".to_owned(),
        }
    }
}

impl From<Color> for Property {
    fn from(color: Color) -> Property {
        Property::Color(color)
    }
}
