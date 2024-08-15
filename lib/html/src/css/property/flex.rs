use super::Property;
use crate::render::Render;

#[derive(Clone)]
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
