mod border;
mod margin;
mod padding;

pub use border::Border;
pub use margin::Margin;
pub use padding::Padding;

use crate::render::Render;

#[derive(Clone)]
pub enum Property {
    Background,
    Color,
    FontFamily,
    FontSize,
    TextAlign,
    Width,
    MaxWidth,
    Height,
    MaxHeight,
    Cursor,
    Display,
    Position,
    AlignContent,
    AlignSelf,
    AlignItems,
    JustifyContent,
    Gap,
    FlexWrap,
    Top,
    Bottom,
    Left,
    Right,
    Overflow,
    Float,
    FlexDirection,
    FontWeight,
    LineHeight,
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
            Property::Background => "background".to_owned(),
            Property::Color => "color".to_owned(),
            Property::FontFamily => "font-family".to_owned(),
            Property::FontSize => "font-size".to_owned(),
            Property::TextAlign => "text-align".to_owned(),
            Property::Width => "width".to_owned(),
            Property::MaxWidth => "max-width".to_owned(),
            Property::Height => "height".to_owned(),
            Property::MaxHeight => "max-height".to_owned(),
            Property::Cursor => "cursor".to_owned(),
            Property::Display => "display".to_owned(),
            Property::Position => "position".to_owned(),
            Property::AlignContent => "align-content".to_owned(),
            Property::AlignSelf => "align-self".to_owned(),
            Property::AlignItems => "align-items".to_owned(),
            Property::JustifyContent => "justify-content".to_owned(),
            Property::Gap => "gap".to_owned(),
            Property::FlexWrap => "flex-wrap".to_owned(),
            Property::Top => "top".to_owned(),
            Property::Bottom => "bottom".to_owned(),
            Property::Left => "left".to_owned(),
            Property::Right => "right".to_owned(),
            Property::Overflow => "overflow".to_owned(),
            Property::Float => "float".to_owned(),
            Property::FlexDirection => "flex-direction".to_owned(),
            Property::FontWeight => "font-weight".to_owned(),
            Property::LineHeight => "line-height".to_owned(),
            Property::Margin(margin) => margin.render(),
            Property::Padding(padding) => padding.render(),
            Property::Border(border) => border.render(),
            Property::Var(v) => format!("--{v}"),
        }
    }
}
