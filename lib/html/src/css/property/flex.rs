use super::Property;
use crate::render::Render;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Flex {
    FlexWrap,
    FlexDirection,
    AlignSelf,
    AlignItems,
    AlignContent,
    JustifyContent,
    Gap,
}

impl Render for Flex {
    fn render(&self) -> String {
        match self {
            Flex::AlignContent => "align-content".to_owned(),
            Flex::AlignItems => "align-items".to_owned(),
            Flex::AlignSelf => "align-self".to_owned(),
            Flex::FlexWrap => "flex-wrap".to_owned(),
            Flex::FlexDirection => "flex-direction".to_owned(),
            Flex::Gap => "gap".to_owned(),
            Flex::JustifyContent => "justify-content".to_owned(),
        }
    }
}

impl From<Flex> for Property {
    fn from(flex: Flex) -> Property {
        Property::Flex(flex)
    }
}

#[cfg(test)]
mod flex_tests {

    use super::{Flex, Property, Render};

    #[test]
    fn render_wrap() {
        let result = Flex::FlexWrap.render();
        let expected = "flex-wrap";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_direction() {
        let result = Flex::FlexDirection.render();
        let expected = "flex-direction";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_alignself() {
        let result = Flex::AlignSelf.render();
        let expected = "align-self";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_aligncontent() {
        let result = Flex::AlignContent.render();
        let expected = "align-content";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_alignitems() {
        let result = Flex::AlignItems.render();
        let expected = "align-items";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_justify() {
        let result = Flex::JustifyContent.render();
        let expected = "justify-content";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_gap() {
        let result = Flex::Gap.render();
        let expected = "gap";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Flex as Into<Property>>::into(Flex::Gap).render();
        let expected = Flex::Gap.render();
        assert_eq!(result, expected)
    }
}
