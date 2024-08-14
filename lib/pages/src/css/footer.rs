pub struct Footer {}

use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Border, Direction, Margin, Property},
    selector::{Selector, SubSelector, TopSelector},
    value::{keyword::Keyword, unit::Unit, Value},
    CssDocument,
};
use std::rc::Rc;

impl CssComponent for Footer {
    fn render(&self) -> CssDocument {
        let footer = CssBlock {
            selector: TopSelector::Id("footer".to_owned()).into(),
            decls: vec![
                (Property::Width, Value::Measurement(100.0, Unit::Percent)).into(),
                (
                    Margin {
                        dir: Direction::Top,
                    }
                    .into(),
                    Value::Measurement(2.0, Unit::Em),
                )
                    .into(),
                (
                    Border {
                        dir: Direction::Top,
                    }
                    .into(),
                    Value::Measurement(0.5, Unit::Em),
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
                        dir: Direction::Right,
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
                    Value::Measurement(0.0, Unit::Px),
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

        let footer_elements = CssBlock {
            selector: Selector {
                top: TopSelector::Id("footer".to_owned()),
                sub: Some(SubSelector::ChildCombinator(Rc::new(
                    TopSelector::All.into(),
                ))),
            },
            decls: vec![
                (
                    Margin {
                        dir: Direction::All,
                    }
                    .into(),
                    Value::Measurement(1.0, Unit::Em),
                )
                    .into(),
                (Property::Float, Keyword::Right.into()).into(),
            ],
        };

        let github = CssBlock {
            selector: TopSelector::Id("github_link".to_owned()).into(),
            decls: vec![(Property::Float, Keyword::Left.into()).into()],
        };

        let image_viewer = CssBlock {
            selector: TopSelector::Id("image_viewer".to_owned()).into(),
            decls: vec![
                (Property::Display, Keyword::Non.into()).into(),
                (Property::Width, Value::Measurement(100.0, Unit::Percent)).into(),
                (Property::Height, Value::Measurement(100.0, Unit::Vh)).into(),
                (Property::Position, Keyword::Fixed.into()).into(),
                (Property::Overflow, Keyword::Non.into()).into(),
                (Property::Left, Value::Measurement(0.0, Unit::Percent)).into(),
                (Property::Top, Value::Measurement(0.0, Unit::Percent)).into(),
                (
                    Margin {
                        dir: Direction::All,
                    }
                    .into(),
                    Value::Measurement(0.0, Unit::Em),
                )
                    .into(),
                (
                    Property::Background,
                    Value::Var("bg-color-trans".to_owned()),
                )
                    .into(),
            ],
        };

        let image_box = CssBlock {
            selector: TopSelector::Id("image_box".to_owned()).into(),
            decls: vec![
                (Property::Position, Keyword::Relative.into()).into(),
                (Property::Width, Value::Measurement(100.0, Unit::Percent)).into(),
                (Property::Height, Value::Measurement(100.0, Unit::Percent)).into(),
                (
                    Margin {
                        dir: Direction::All,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
                (
                    Margin {
                        dir: Direction::Top,
                    }
                    .into(),
                    Value::Measurement(5.0, Unit::Percent),
                )
                    .into(),
                (Property::Overflow, Keyword::Auto.into()).into(),
            ],
        };

        let image_viewer_image = CssBlock {
            selector: TopSelector::Id("image_viewer_image".to_owned()).into(),
            decls: vec![
                (Property::Display, Keyword::Block.into()).into(),
                (
                    Margin {
                        dir: Direction::Left,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
                (
                    Margin {
                        dir: Direction::Right,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
                (
                    Margin {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
                (
                    Margin {
                        dir: Direction::Top,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
                (Property::MaxWidth, Value::Measurement(90.0, Unit::Percent)).into(),
                (Property::MaxHeight, Value::Measurement(80.0, Unit::Percent)).into(),
                (Property::Width, Keyword::Auto.into()).into(),
                (Property::Height, Keyword::Auto.into()).into(),
            ],
        };

        CssDocument {
            decls: vec![
                footer,
                footer_elements,
                github,
                image_viewer,
                image_box,
                image_viewer_image,
            ],
        }
    }
}
