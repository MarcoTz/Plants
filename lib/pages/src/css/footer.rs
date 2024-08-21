pub struct Footer {}

use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Border, Color, Direction, Margin, Position, Property, Size},
    selector::{Selector, SubSelector, TopSelector},
    value::{Keyword, Unit, Value},
    CssDocument,
};
use log;
use std::rc::Rc;

impl CssComponent for Footer {
    fn render(&self) -> CssDocument {
        log::info!("Loading footer css");
        let footer = CssBlock {
            selector: TopSelector::Id("footer".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
                (
                    Margin {
                        dir: Direction::Top,
                    }
                    .into(),
                    (2.0, Unit::Em).into(),
                )
                    .into(),
                (Border::Side(Direction::Top).into(), (0.5, Unit::Em).into()).into(),
                (Border::Side(Direction::Top).into(), (0.0, Unit::Px).into()).into(),
                (
                    Border::Side(Direction::Right).into(),
                    (0.0, Unit::Px).into(),
                )
                    .into(),
                (
                    Border::Side(Direction::Bottom).into(),
                    (0.0, Unit::Px).into(),
                )
                    .into(),
                (Border::Style.into(), Keyword::Solid.into()).into(),
                (Border::Color.into(), Value::Var("bg-color-even".to_owned())).into(),
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
                    (1.0, Unit::Em).into(),
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
                (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
                (Size::Height.into(), (100.0, Unit::Vh).into()).into(),
                (Position::Position.into(), Keyword::Fixed.into()).into(),
                (Property::Overflow, Keyword::Non.into()).into(),
                (Position::Left.into(), (0.0, Unit::Percent).into()).into(),
                (Position::Top.into(), (0.0, Unit::Percent).into()).into(),
                (
                    Margin {
                        dir: Direction::All,
                    }
                    .into(),
                    (0.0, Unit::Em).into(),
                )
                    .into(),
                (
                    Color::Background.into(),
                    Value::Var("bg-color-trans".to_owned()),
                )
                    .into(),
            ],
        };

        let image_box = CssBlock {
            selector: TopSelector::Id("image_box".to_owned()).into(),
            decls: vec![
                (Position::Position.into(), Keyword::Relative.into()).into(),
                (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
                (Size::Height.into(), (100.0, Unit::Percent).into()).into(),
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
                    (5.0, Unit::Percent).into(),
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
                (Size::MaxWidth.into(), (90.0, Unit::Percent).into()).into(),
                (Size::MaxHeight.into(), (80.0, Unit::Percent).into()).into(),
                (Size::Width.into(), Keyword::Auto.into()).into(),
                (Size::Height.into(), Keyword::Auto.into()).into(),
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
