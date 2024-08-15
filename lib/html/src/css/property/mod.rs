mod border;
mod color;
mod flex;
mod font;
mod margin;
mod padding;
mod position;
mod size;

pub use border::Border;
pub use color::Color;
pub use flex::Flex;
pub use font::Font;
pub use margin::Margin;
pub use padding::Padding;
pub use position::Position;
pub use size::Size;

use crate::render::Render;

#[derive(Clone)]
pub enum Property {
    TextAlign,
    Cursor,
    Display,
    Overflow,
    Float,
    LineHeight,
    Position(Position),
    Color(Color),
    Size(Size),
    Flex(Flex),
    Font(Font),
    Margin(Margin),
    Padding(Padding),
    Var(String),
    Border(Border),
}

#[derive(Clone)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    All,
}

impl Render for Property {
    fn render(&self) -> String {
        match self {
            Property::TextAlign => "text-align".to_owned(),
            Property::Cursor => "cursor".to_owned(),
            Property::Display => "display".to_owned(),
            Property::Overflow => "overflow".to_owned(),
            Property::Float => "float".to_owned(),
            Property::LineHeight => "line-height".to_owned(),
            Property::Position(pos) => pos.render(),
            Property::Color(color) => color.render(),
            Property::Size(size) => size.render(),
            Property::Flex(flex) => flex.render(),
            Property::Font(font) => font.render(),
            Property::Margin(margin) => margin.render(),
            Property::Padding(padding) => padding.render(),
            Property::Border(border) => border.render(),
            Property::Var(v) => format!("--{v}"),
        }
    }
}
