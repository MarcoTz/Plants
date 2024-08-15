use super::Property;
use crate::render::Render;

#[derive(Clone)]
pub enum Font {
    Weight,
    Family,
    Size,
}

impl Render for Font {
    fn render(&self) -> String {
        match self {
            Font::Family => "font-family".to_owned(),
            Font::Weight => "font-weight".to_owned(),
            Font::Size => "font-size".to_owned(),
        }
    }
}

impl From<Font> for Property {
    fn from(font: Font) -> Property {
        Property::Font(font)
    }
}
