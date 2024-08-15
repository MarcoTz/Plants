use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Border, Direction, Margin, Padding, Property},
    selector::{Selector, SubSelector, TopSelector},
    value::{keyword::Keyword, unit::Unit, Value},
    CssDocument,
};
use std::rc::Rc;

pub struct Header {}

impl CssComponent for Header {
    fn render(&self) -> CssDocument {
        let header_items = CssBlock {
            selector: Selector {
                top: TopSelector::Id("header".to_owned()),
                sub: Some(SubSelector::ChildCombinator(Rc::new(
                    TopSelector::All.into(),
                ))),
            },
            decls: vec![
                (Property::FontSize, (16.0, Unit::Pt).into()).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Border::Radius.into(), (1.0, Unit::Em).into()).into(),
                (Property::Width, (10.0, Unit::Em).into()).into(),
                (Property::Height, (3.0, Unit::Em).into()).into(),
                (
                    Padding {
                        dir: Direction::All,
                    }
                    .into(),
                    (0.0, Unit::Em).into(),
                )
                    .into(),
                (
                    Padding {
                        dir: Direction::Top,
                    }
                    .into(),
                    (1.0, Unit::Em).into(),
                )
                    .into(),
            ],
        };

        let header = CssBlock {
            selector: TopSelector::Id("header".to_owned()).into(),
            decls: vec![
                (Property::Width, (100.0, Unit::Percent).into()).into(),
                (
                    Margin {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    (2.0, Unit::Em).into(),
                )
                    .into(),
                (
                    Margin {
                        dir: Direction::Top,
                    }
                    .into(),
                    (2.0, Unit::Em).into(),
                )
                    .into(),
                (
                    Padding {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    (1.0, Unit::Em).into(),
                )
                    .into(),
                (Border::Side(Direction::Top).into(), (0.0, Unit::Px).into()).into(),
                (
                    Border::Side(Direction::Right).into(),
                    (0.0, Unit::Px).into(),
                )
                    .into(),
                (Border::Side(Direction::Left).into(), (0.0, Unit::Px).into()).into(),
                (
                    Border::Side(Direction::Bottom).into(),
                    (0.5, Unit::Em).into(),
                )
                    .into(),
                (Border::Style.into(), Keyword::Solid.into()).into(),
                (Border::Color.into(), Value::Var("bg-color-even".to_owned())).into(),
            ],
        };

        CssDocument {
            decls: vec![header_items, header],
        }
    }
}
