use super::Value;
use crate::render::Render;

#[derive(Clone, PartialEq, Debug)]
pub enum Color {
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f32),
}

impl Render for Color {
    fn render(&self) -> String {
        match self {
            Color::Rgb(r, g, b) => {
                format!("#{r:02x}{g:02x}{b:02x}")
            }
            Color::Rgba(r, g, b, a) => format!("rgba({r},{g},{b},{a:.2})"),
        }
    }
}

impl From<Color> for Value {
    fn from(color: Color) -> Value {
        Value::Color(color)
    }
}

#[cfg(test)]
mod color_tests {

    use super::{Color, Render, Value};

    #[test]
    fn render_rgb() {
        let result = Color::Rgb(5, 10, 16).render();
        let expected = "#050a10";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_rgba() {
        let result = Color::Rgba(10, 15, 20, 10.581).render();
        let expected = "rgba(10,15,20,10.58)";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Color as Into<Value>>::into(Color::Rgb(10, 10, 10)).render();
        let expected = Color::Rgb(10, 10, 10).render();
        assert_eq!(result, expected)
    }
}
