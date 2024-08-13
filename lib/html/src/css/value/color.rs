use crate::render::Render;

pub enum Color {
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8),
}

impl Render for Color {
    fn render(&self) -> String {
        match self {
            Color::Rgb(r, g, b) => format!("#{r:x}{g:x}{b:x}"),
            Color::Rgba(r, g, b, a) => format!("rgba({r},{g},{b},{a})"),
        }
    }
}
