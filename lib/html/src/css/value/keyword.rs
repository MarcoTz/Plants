use super::Value;
use crate::render::Render;

#[derive(Clone, PartialEq, Eq, Debug)]
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
    FlowRoot,
    NoWrap,
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
            Keyword::FlowRoot => "flot-root".to_owned(),
            Keyword::NoWrap => "nowrap".to_owned(),
        }
    }
}

impl From<Keyword> for Value {
    fn from(keyword: Keyword) -> Value {
        Value::Keyword(keyword)
    }
}

#[cfg(test)]
mod keyword_tests {

    use super::{Keyword, Render, Value};

    #[test]
    fn render_center() {
        let result = Keyword::Center.render();
        let expected = "center";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_flexstart() {
        let result = Keyword::FlexStart.render();
        let expected = "flex-start";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_pointer() {
        let result = Keyword::Pointer.render();
        let expected = "pointer";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_collapse() {
        let result = Keyword::Collapse.render();
        let expected = "collapse";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_auto() {
        let result = Keyword::Auto.render();
        let expected = "auto";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_right() {
        let result = Keyword::Right.render();
        let expected = "right";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_left() {
        let result = Keyword::Left.render();
        let expected = "left";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_flex() {
        let result = Keyword::Flex.render();
        let expected = "flex";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_spacearound() {
        let result = Keyword::SpaceAround.render();
        let expected = "space-around";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_wrap() {
        let result = Keyword::Wrap.render();
        let expected = "wrap";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_stretch() {
        let result = Keyword::Stretch.render();
        let expected = "stretch";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_none() {
        let result = Keyword::Non.render();
        let expected = "none";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_fixed() {
        let result = Keyword::Fixed.render();
        let expected = "fixed";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_relative() {
        let result = Keyword::Relative.render();
        let expected = "relative";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_block() {
        let result = Keyword::Block.render();
        let expected = "block";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_solid() {
        let result = Keyword::Solid.render();
        let expected = "solid";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_column() {
        let result = Keyword::Column.render();
        let expected = "column";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_flexend() {
        let result = Keyword::FlexEnd.render();
        let expected = "flex-end";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_bold() {
        let result = Keyword::Bold.render();
        let expected = "bold";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_transparent() {
        let result = Keyword::Transparent.render();
        let expected = "transparent";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_hidden() {
        let result = Keyword::Hidden.render();
        let expected = "hidden";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_flowroot() {
        let result = Keyword::FlowRoot.render();
        let expected = "flot-root";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_nowrap() {
        let result = Keyword::NoWrap.render();
        let expected = "nowrap";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Keyword as Into<Value>>::into(Keyword::Center).render();
        let expected = Keyword::Center.render();
        assert_eq!(result, expected)
    }
}
