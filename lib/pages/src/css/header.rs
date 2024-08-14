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
                (Property::FontSize, Value::Measurement(16.0, Unit::Pt)).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Property::BorderRadius, Value::Measurement(1.0, Unit::Em)).into(),
                (Property::Width, Value::Measurement(10.0, Unit::Em)).into(),
                (Property::Height, Value::Measurement(3.0, Unit::Em)).into(),
                (
                    Padding {
                        dir: Direction::All,
                    }
                    .into(),
                    Value::Measurement(0.0, Unit::Em),
                )
                    .into(),
                (
                    Padding {
                        dir: Direction::Top,
                    }
                    .into(),
                    Value::Measurement(1.0, Unit::Em),
                )
                    .into(),
            ],
        };

        let header = CssBlock {
            selector: TopSelector::Id("header".to_owned()).into(),
            decls: vec![
                (Property::Width, Value::Measurement(100.0, Unit::Percent)).into(),
                (
                    Margin {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    Value::Measurement(2.0, Unit::Em),
                )
                    .into(),
                (
                    Margin {
                        dir: Direction::Top,
                    }
                    .into(),
                    Value::Measurement(2.0, Unit::Em),
                )
                    .into(),
                (
                    Padding {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    Value::Measurement(1.0, Unit::Em),
                )
                    .into(),
                (
                    Border {
                        dir: Direction::Top,
                    }
                    .into(),
                    Value::Measurement(0.0, Unit::Px),
                )
                    .into(),
                (
                    Border {
                        dir: Direction::Right,
                    }
                    .into(),
                    Value::Measurement(0.0, Unit::Px),
                )
                    .into(),
                (
                    Border {
                        dir: Direction::Left,
                    }
                    .into(),
                    Value::Measurement(0.0, Unit::Px),
                )
                    .into(),
                (
                    Border {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    Value::Measurement(0.5, Unit::Em),
                )
                    .into(),
                (Property::BorderStyle, Keyword::Solid.into()).into(),
                (
                    Property::BorderColor,
                    Value::Var("bg-color-even".to_owned()),
                )
                    .into(),
            ],
        };

        CssDocument {
            decls: vec![header_items, header],
        }
    }
}
