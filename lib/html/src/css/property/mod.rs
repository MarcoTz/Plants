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

#[derive(Clone, PartialEq, Eq, Debug)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod property_tests {
    use super::{
        Border, Color, Direction, Flex, Font, Margin, Padding, Position, Property, Render, Size,
    };

    #[test]
    fn render_align() {
        let result = Property::TextAlign.render();
        let expected = "text-align";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_cursor() {
        let result = Property::Cursor.render();
        let expected = "cursor";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_display() {
        let result = Property::Display.render();
        let expected = "display";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_float() {
        let result = Property::Float.render();
        let expected = "float";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_lineheight() {
        let result = Property::LineHeight.render();
        let expected = "line-height";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_position() {
        let result = Property::Position(Position::Top).render();
        let expected = "top";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_color() {
        let result = Property::Color(Color::Background).render();
        let expected = "background";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_size() {
        let result = Property::Size(Size::Width).render();
        let expected = "width";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_flex() {
        let result = Property::Flex(Flex::Gap).render();
        let expected = "gap";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_font() {
        let result = Property::Font(Font::Size).render();
        let expected = "font-size";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_margin() {
        let result = Property::Margin(Margin {
            dir: Direction::Top,
        })
        .render();
        let expected = "margin-top";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_padding() {
        let result = Property::Padding(Padding {
            dir: Direction::Bottom,
        })
        .render();
        let expected = "padding-bottom";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_border() {
        let result = Property::Border(Border::Radius).render();
        let expected = "border-radius";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_var() {
        let result = Property::Var("bg-color".to_owned()).render();
        let expected = "--bg-color";
        assert_eq!(result, expected)
    }
}
