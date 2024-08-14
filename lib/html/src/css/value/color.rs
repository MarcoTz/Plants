use super::Value;
use crate::render::Render;

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
