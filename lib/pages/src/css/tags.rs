pub struct Tags {}
use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property,
    property::{Border, Direction, Flex, Font, Margin, Padding, Property, Size},
    selector::{ChildSelector, Selector, SubSelector, TopSelector},
    value::{Keyword, Unit, Value},
    CssDocument,
};

impl CssComponent for Tags {
    fn render(&self) -> CssDocument {
        log::info!("Loading common CSS for Html Tags");
        let body = CssBlock {
            selector: TopSelector::Tag("body".to_owned()).into(),
            decls: vec![
                (
                    property::Color::Background.into(),
                    Value::Var("bg-color".to_owned()),
                )
                    .into(),
                (
                    property::Color::Color.into(),
                    Value::Var("fg-color".to_owned()),
                )
                    .into(),
                (Font::Family.into(), Value::Str("Noto Sans".to_owned())).into(),
                (Font::Size.into(), (14.0, Unit::Pt).into()).into(),
            ],
        };

        let h1 = CssBlock {
            selector: TopSelector::Tag("h1".to_owned()).into(),
            decls: vec![
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
            ],
        };

        let h2 = CssBlock {
            selector: TopSelector::Tag("h2".to_owned()).into(),
            decls: vec![
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
                (Flex::AlignSelf.into(), Keyword::FlexStart.into()).into(),
            ],
        };

        let h3 = CssBlock {
            selector: TopSelector::Tag("h3".to_owned()).into(),
            decls: vec![
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
            ],
        };

        let a = CssBlock {
            selector: TopSelector::Tag("a".to_owned()).into(),
            decls: vec![(
                property::Color::Color.into(),
                Value::Var("link-color".to_owned()),
            )
                .into()],
        };

        let a_visited = CssBlock {
            selector: Selector {
                top: TopSelector::Tag("a".to_owned()),
                sub: Some(SubSelector::Visited),
            },
            decls: vec![(
                property::Color::Color.into(),
                Value::Var("link-color-visited".to_owned()),
            )
                .into()],
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
            decls: vec![(
                property::Color::Background.into(),
                Value::Var("bg-color-odd".to_owned()),
            )
                .into()],
        };

        let tr_even = CssBlock {
            selector: Selector {
                top: TopSelector::Tag("tr".to_owned()),
                sub: Some(ChildSelector::Even.into()),
            },
            decls: vec![(
                property::Color::Background.into(),
                Value::Var("bg-color-even".to_owned()),
            )
                .into()],
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
                (
                    property::Color::Background.into(),
                    Value::Var("bg-color".to_owned()),
                )
                    .into(),
                (
                    property::Color::Color.into(),
                    Value::Var("fg-color".to_owned()),
                )
                    .into(),
            ],
        };

        let select = CssBlock {
            selector: TopSelector::Tag("select".to_owned()).into(),
            decls: vec![
                (
                    property::Color::Background.into(),
                    Value::Var("bg-color".to_owned()),
                )
                    .into(),
                (
                    property::Color::Color.into(),
                    Value::Var("fg-color".to_owned()),
                )
                    .into(),
            ],
        };

        CssDocument {
            decls: vec![
                body, h1, h2, h3, a, a_visited, img, table, tr_odd, tr_even, td, td_even, td_odd,
                input, select,
            ],
        }
    }
}
