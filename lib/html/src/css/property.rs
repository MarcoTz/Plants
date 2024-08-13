use crate::render::Render;

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
    BorderCollapse,
    Margin(Margin),
    Padding(Padding),
    Display,
    Position,
    AlignContent,
    AlignSelf,
    AlignItems,
    JustifyContent,
    Gap,
    FlexWrap,
    BorderRadius,
    Top,
    Bottom,
    Left,
    Right,
    Overflow,
    Var(String),
}

pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    All,
}

pub struct Margin {
    pub dir: Direction,
}
pub struct Padding {
    pub dir: Direction,
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
            Property::BorderCollapse => "border-collapse".to_owned(),
            Property::Margin(margin) => margin.render(),
            Property::Padding(padding) => padding.render(),
            Property::Display => "display".to_owned(),
            Property::Position => "position".to_owned(),
            Property::AlignContent => "align-content".to_owned(),
            Property::AlignSelf => "align-self".to_owned(),
            Property::AlignItems => "align-items".to_owned(),
            Property::JustifyContent => "justify-content".to_owned(),
            Property::Gap => "gap".to_owned(),
            Property::FlexWrap => "flex-wrap".to_owned(),
            Property::BorderRadius => "border-radius".to_owned(),
            Property::Top => "top".to_owned(),
            Property::Bottom => "bottom".to_owned(),
            Property::Left => "left".to_owned(),
            Property::Right => "right".to_owned(),
            Property::Overflow => "overflow".to_owned(),
            Property::Var(v) => format!("--{v}"),
        }
    }
}

impl Render for Margin {
    fn render(&self) -> String {
        match self.dir {
            Direction::Top => "margin-top".to_owned(),
            Direction::Bottom => "margin-bottom".to_owned(),
            Direction::Left => "margin-left".to_owned(),
            Direction::Right => "margin-right".to_owned(),
            Direction::All => "margin".to_owned(),
        }
    }
}

impl Render for Padding {
    fn render(&self) -> String {
        match self.dir {
            Direction::Top => "padding-top".to_owned(),
            Direction::Bottom => "padding-bottom".to_owned(),
            Direction::Left => "padding-left".to_owned(),
            Direction::Right => "padding-right".to_owned(),
            Direction::All => "padding".to_owned(),
        }
    }
}
