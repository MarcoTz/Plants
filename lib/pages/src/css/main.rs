use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Border, Direction, Margin, Padding, Property},
    selector::{ChildSelector, Selector, SubSelector, TopSelector},
    value::{color::Color, keyword::Keyword, unit::Unit, Value},
    CssDocument,
};
use std::rc::Rc;

pub struct Main {}

impl CssComponent for Main {
    fn render(&self) -> CssDocument {
        let root = CssBlock {
            selector: TopSelector::Pseudo("root".to_owned()).into(),
            decls: vec![
                (
                    Property::Var("bg-color".to_owned()),
                    Color::Rgb(34, 35, 39).into(),
                )
                    .into(),
                (
                    Property::Var("fg-color".to_owned()),
                    Color::Rgb(102, 204, 224).into(),
                )
                    .into(),
                (
                    Property::Var("fg-color-dark".to_owned()),
                    Color::Rgb(51, 102, 112).into(),
                )
                    .into(),
                (
                    Property::Var("link-color".to_owned()),
                    Color::Rgb(158, 208, 114).into(),
                )
                    .into(),
                (
                    Property::Var("link-color-visited".to_owned()),
                    Color::Rgb(114, 162, 120).into(),
                )
                    .into(),
                (
                    Property::Var("bg-color-odd".to_owned()),
                    Color::Rgb(42, 45, 55).into(),
                )
                    .into(),
                (
                    Property::Var("bg-color-even".to_owned()),
                    Color::Rgb(66, 86, 97).into(),
                )
                    .into(),
                (
                    Property::Var("bg-color-trans".to_owned()),
                    Color::Rgba(22, 23, 27, 0.8).into(),
                )
                    .into(),
                (
                    Property::Var("health-color0".to_owned()),
                    Color::Rgb(117, 118, 117).into(),
                )
                    .into(),
                (
                    Property::Var("health-color1".to_owned()),
                    Color::Rgb(214, 10, 14).into(),
                )
                    .into(),
                (
                    Property::Var("health-color2".to_owned()),
                    Color::Rgb(249, 123, 14).into(),
                )
                    .into(),
                (
                    Property::Var("health-color3".to_owned()),
                    Color::Rgb(217, 224, 8).into(),
                )
                    .into(),
                (
                    Property::Var("health-color4".to_owned()),
                    Color::Rgb(146, 229, 19).into(),
                )
                    .into(),
                (
                    Property::Var("health-color5".to_owned()),
                    Color::Rgb(51, 216, 24).into(),
                )
                    .into(),
            ],
        };

        let body = CssBlock {
            selector: TopSelector::Tag("body".to_owned()).into(),
            decls: vec![
                (Property::Background, Value::Var("bg-color".to_owned())).into(),
                (Property::Color, Value::Var("fg-color".to_owned())).into(),
                (Property::FontFamily, Value::Str("Noto Sans".to_owned())).into(),
                (Property::FontSize, (14.0, Unit::Pt).into()).into(),
            ],
        };

        let h1 = CssBlock {
            selector: TopSelector::Tag("h1".to_owned()).into(),
            decls: vec![
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Property::Width, (100.0, Unit::Percent).into()).into(),
            ],
        };

        let h2 = CssBlock {
            selector: TopSelector::Tag("h2".to_owned()).into(),
            decls: vec![
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Property::Width, (100.0, Unit::Percent).into()).into(),
                (Property::AlignSelf, Keyword::FlexStart.into()).into(),
            ],
        };

        let h3 = CssBlock {
            selector: TopSelector::Tag("h3".to_owned()).into(),
            decls: vec![
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Property::Width, (100.0, Unit::Percent).into()).into(),
            ],
        };

        let a = CssBlock {
            selector: TopSelector::Tag("a".to_owned()).into(),
            decls: vec![(Property::Color, Value::Var("link-color".to_owned())).into()],
        };

        let a_visited = CssBlock {
            selector: Selector {
                top: TopSelector::Tag("a".to_owned()),
                sub: Some(SubSelector::Visited),
            },
            decls: vec![(Property::Color, Value::Var("link-color-visited".to_owned())).into()],
        };

        let img = CssBlock {
            selector: TopSelector::Tag("img".to_owned()).into(),
            decls: vec![(Property::Cursor, Keyword::Pointer.into()).into()],
        };

        let table = CssBlock {
            selector: TopSelector::Tag("table".to_owned()).into(),
            decls: vec![
                (Border::Collapse.into(), Keyword::Collapse.into()).into(),
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

        let tr_odd = CssBlock {
            selector: Selector {
                top: TopSelector::Tag("tr".to_owned()),
                sub: Some(ChildSelector::Odd.into()),
            },
            decls: vec![(Property::Background, Value::Var("bg-color-odd".to_owned())).into()],
        };
        let tr_even = CssBlock {
            selector: Selector {
                top: TopSelector::Tag("tr".to_owned()),
                sub: Some(ChildSelector::Even.into()),
            },
            decls: vec![(Property::Background, Value::Var("bg-color-even".to_owned())).into()],
        };

        let td = CssBlock {
            selector: TopSelector::Tag("td".to_owned()).into(),
            decls: vec![
                (
                    Padding {
                        dir: Direction::Left,
                    }
                    .into(),
                    (1.0, Unit::Em).into(),
                )
                    .into(),
                (
                    Padding {
                        dir: Direction::Right,
                    }
                    .into(),
                    (1.0, Unit::Em).into(),
                )
                    .into(),
            ],
        };

        let td_even = CssBlock {
            selector: Selector {
                top: TopSelector::Tag("td".to_owned()),
                sub: Some(ChildSelector::Even.into()),
            },
            decls: vec![(Property::TextAlign, Keyword::Left.into()).into()],
        };

        let td_odd = CssBlock {
            selector: Selector {
                top: TopSelector::Tag("td".to_owned()),
                sub: Some(ChildSelector::Odd.into()),
            },
            decls: vec![(Property::TextAlign, Keyword::Right.into()).into()],
        };

        let input = CssBlock {
            selector: TopSelector::Tag("input".to_owned()).into(),
            decls: vec![
                (Property::Background, Value::Var("bg-color".to_owned())).into(),
                (Property::Color, Value::Var("fg-color".to_owned())).into(),
            ],
        };

        let select = CssBlock {
            selector: TopSelector::Tag("select".to_owned()).into(),
            decls: vec![
                (Property::Background, Value::Var("bg-color".to_owned())).into(),
                (Property::Color, Value::Var("fg-color".to_owned())).into(),
            ],
        };

        let alternating_children_even = CssBlock {
            selector: Selector {
                top: TopSelector::Class("alternating_children".to_owned()),
                sub: Some(SubSelector::ChildCombinator(Rc::new(Selector {
                    top: TopSelector::All,
                    sub: Some(ChildSelector::Even.into()),
                }))),
            },
            decls: vec![(Property::Background, Value::Var("bg-color-even".to_owned())).into()],
        };

        let alternating_children_odd = CssBlock {
            selector: Selector {
                top: TopSelector::Class("alternating_children".to_owned()),
                sub: Some(SubSelector::ChildCombinator(Rc::new(Selector {
                    top: TopSelector::All,
                    sub: Some(ChildSelector::Odd.into()),
                }))),
            },
            decls: vec![(Property::Background, Value::Var("bg-color-odd".to_owned())).into()],
        };

        let flex_container = CssBlock {
            selector: TopSelector::Class("flex_container".to_owned()).into(),
            decls: vec![
                (Property::Display, Keyword::Flex.into()).into(),
                (Property::AlignContent, Keyword::SpaceAround.into()).into(),
                (Property::AlignSelf, Keyword::Center.into()).into(),
                (Property::JustifyContent, Keyword::Center.into()).into(),
                (Property::Gap, (1.0, Unit::Em).into()).into(),
                (Property::FlexWrap, Keyword::Wrap.into()).into(),
                (Property::AlignItems, Keyword::Stretch.into()).into(),
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
                root,
                body,
                h1,
                h2,
                h3,
                a,
                a_visited,
                img,
                table,
                tr_odd,
                tr_even,
                td,
                td_odd,
                td_even,
                input,
                select,
                alternating_children_odd,
                alternating_children_even,
                flex_container,
                flex_container_children,
            ],
        }
    }
}
