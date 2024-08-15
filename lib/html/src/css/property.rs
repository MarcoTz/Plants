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
    BorderCollapse,
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
    BorderStyle,
    BorderColor,
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

#[derive(Clone)]
pub struct Margin {
    pub dir: Direction,
}

#[derive(Clone)]
pub struct Padding {
    pub dir: Direction,
}

#[derive(Clone)]
pub struct Border {
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
            Property::BorderStyle => "border-style".to_owned(),
            Property::BorderColor => "border-color".to_owned(),
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

impl Render for Border {
    fn render(&self) -> String {
        match self.dir {
            Direction::Top => "border-top".to_owned(),
            Direction::Bottom => "border-bottom".to_owned(),
            Direction::Left => "border-left".to_owned(),
            Direction::Right => "border-right".to_owned(),
            Direction::All => "border".to_owned(),
        }
    }
}

impl From<Margin> for Property {
    fn from(margin: Margin) -> Property {
        Property::Margin(margin)
    }
}

impl From<Padding> for Property {
    fn from(padding: Padding) -> Property {
        Property::Padding(padding)
    }
}

impl From<Border> for Property {
    fn from(border: Border) -> Property {
        Property::Border(border)
    }
}
