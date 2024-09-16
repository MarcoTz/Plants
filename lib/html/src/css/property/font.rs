use super::Property;
use crate::render::Render;

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[cfg(test)]
mod font_tests {

    use super::{Font, Property, Render};

    #[test]
    fn render_weight() {
        let result = Font::Weight.render();
        let expected = "font-weight";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_family() {
        let result = Font::Family.render();
        let expected = "font-family";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_size() {
        let result = Font::Size.render();
        let expected = "font-size";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Font as Into<Property>>::into(Font::Size).render();
        let expected = Font::Size.render();
        assert_eq!(result, expected)
    }
}
