use super::Property;
use crate::render::Render;

#[derive(Clone, PartialEq, Eq, Debug)]
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

#[cfg(test)]
mod color_tests {
    use super::{Color, Property, Render};

    #[test]
    fn render_background() {
        let result = Color::Background.render();
        let expected = "background";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_color() {
        let result = Color::Color.render();
        let expected = "color";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Color as Into<Property>>::into(Color::Color).render();
        let expected = Color::Color.render();
        assert_eq!(result, expected)
    }
}
