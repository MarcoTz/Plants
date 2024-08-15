use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property,
    property::{Border, Direction, Flex, Margin, Padding, Property},
    selector::{ChildSelector, Selector, SubSelector, TopSelector},
    value::{Keyword, Unit, Value},
    CssDocument,
};
use std::rc::Rc;

pub struct Classes {}

impl CssComponent for Classes {
    fn render(&self) -> CssDocument {
        let alternating_children_even = CssBlock {
            selector: Selector {
                top: TopSelector::Class("alternating_children".to_owned()),
                sub: Some(SubSelector::ChildCombinator(Rc::new(Selector {
                    top: TopSelector::All,
                    sub: Some(ChildSelector::Even.into()),
                }))),
            },
            decls: vec![(
                property::Color::Background.into(),
                Value::Var("bg-color-even".to_owned()),
            )
                .into()],
        };

        let alternating_children_odd = CssBlock {
            selector: Selector {
                top: TopSelector::Class("alternating_children".to_owned()),
                sub: Some(SubSelector::ChildCombinator(Rc::new(Selector {
                    top: TopSelector::All,
                    sub: Some(ChildSelector::Odd.into()),
                }))),
            },
            decls: vec![(
                property::Color::Background.into(),
                Value::Var("bg-color-odd".to_owned()),
            )
                .into()],
        };

        let flex_container = CssBlock {
            selector: TopSelector::Class("flex_container".to_owned()).into(),
            decls: vec![
                (Property::Display, Keyword::Flex.into()).into(),
                (Flex::AlignContent.into(), Keyword::SpaceAround.into()).into(),
                (Flex::AlignSelf.into(), Keyword::Center.into()).into(),
                (Flex::JustifyContent.into(), Keyword::Center.into()).into(),
                (Flex::Gap.into(), (1.0, Unit::Em).into()).into(),
                (Flex::FlexWrap.into(), Keyword::Wrap.into()).into(),
                (Flex::AlignItems.into(), Keyword::Stretch.into()).into(),
                (
                    Margin {
                        dir: Direction::All,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
            ],
        };

        let flex_container_children = CssBlock {
            selector: Selector {
                top: TopSelector::Class("flex_container".to_owned()),
                sub: Some(SubSelector::ChildCombinator(Rc::new(
                    TopSelector::All.into(),
                ))),
            },
            decls: vec![
                (Border::Radius.into(), (1.0, Unit::Em).into()).into(),
                (
                    Padding {
                        dir: Direction::All,
                    }
                    .into(),
                    (1.0, Unit::Em).into(),
                )
                    .into(),
            ],
        };

        CssDocument {
            decls: vec![
                alternating_children_odd,
                alternating_children_even,
                flex_container,
                flex_container_children,
            ],
        }
    }
}
