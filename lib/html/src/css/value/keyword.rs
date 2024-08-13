use crate::render::Render;

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
        }
    }
}
