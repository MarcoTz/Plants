use super::Value;
use crate::render::Render;

#[derive(Clone)]
pub enum Keyword {
    Center,
    FlexStart,
    Pointer,
    Collapse,
    Auto,
    Right,
    Left,
    Flex,
    SpaceAround,
    Wrap,
    Stretch,
    Non,
    Fixed,
    Relative,
    Block,
    Solid,
    Column,
    FlexEnd,
    Bold,
    Transparent,
    Hidden,
}

impl Render for Keyword {
    fn render(&self) -> String {
        match self {
            Keyword::Center => "center".to_owned(),
            Keyword::FlexStart => "flex-start".to_owned(),
            Keyword::Pointer => "pointer".to_owned(),
            Keyword::Collapse => "collapse".to_owned(),
            Keyword::Auto => "auto".to_owned(),
            Keyword::Right => "right".to_owned(),
            Keyword::Left => "left".to_owned(),
            Keyword::Flex => "flex".to_owned(),
            Keyword::SpaceAround => "space-around".to_owned(),
            Keyword::Wrap => "wrap".to_owned(),
            Keyword::Stretch => "stretch".to_owned(),
            Keyword::Non => "none".to_owned(),
            Keyword::Fixed => "fixed".to_owned(),
            Keyword::Relative => "relative".to_owned(),
            Keyword::Block => "block".to_owned(),
            Keyword::Solid => "solid".to_owned(),
            Keyword::Column => "column".to_owned(),
            Keyword::FlexEnd => "flex-end".to_owned(),
            Keyword::Bold => "bold".to_owned(),
            Keyword::Transparent => "transparent".to_owned(),
            Keyword::Hidden => "hidden".to_owned(),
        }
    }
}

impl From<Keyword> for Value {
    fn from(keyword: Keyword) -> Value {
        Value::Keyword(keyword)
    }
}
