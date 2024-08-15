use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Direction, Flex, Font, Margin, Padding, Property, Size},
    selector::TopSelector,
    value::{Keyword, Unit},
    CssDocument,
};

pub struct Gallery {}

impl CssComponent for Gallery {
    fn render(&self) -> CssDocument {
        let plant_gallery = CssBlock {
            selector: TopSelector::Id("plant_gallery".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (95.0, Unit::Percent).into()).into(),
                (
                    Padding {
                        dir: Direction::All,
                    }
                    .into(),
                    (0.0, Unit::Em).into(),
                )
                    .into(),
            ],
        };

        let images_plant_container = CssBlock {
            selector: TopSelector::Class("images_plant_container".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (19.0, Unit::Percent).into()).into(),
                (Flex::FlexDirection.into(), Keyword::Column.into()).into(),
                (
                    Padding {
                        dir: Direction::All,
                    }
                    .into(),
                    (0.0, Unit::Em).into(),
                )
                    .into(),
            ],
        };

        let images_plant = CssBlock {
            selector: TopSelector::Class("images_plant".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (90.0, Unit::Percent).into()).into(),
                (
                    Margin {
                        dir: Direction::All,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
                (
                    Padding {
                        dir: Direction::All,
                    }
                    .into(),
                    (0.0, Unit::Em).into(),
                )
                    .into(),
            ],
        };

        let plant_image = CssBlock {
            selector: TopSelector::Class("plant_image".to_owned()).into(),
            decls: vec![(
                Margin {
                    dir: Direction::All,
                }
                .into(),
                Keyword::Auto.into(),
            )
                .into()],
        };

        let img = CssBlock {
            selector: TopSelector::Tag("img".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
                (Size::Height.into(), (100.0, Unit::Percent).into()).into(),
            ],
        };

        let h2 = CssBlock {
            selector: TopSelector::Tag("h2".to_owned()).into(),
            decls: vec![(
                Margin {
                    dir: Direction::All,
                }
                .into(),
                (0.0, Unit::Em).into(),
            )
                .into()],
        };

        let img_controls = CssBlock {
            selector: TopSelector::Class("img_controls".to_owned()).into(),
            decls: vec![
                (Flex::AlignSelf.into(), Keyword::FlexEnd.into()).into(),
                (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
                (
                    Padding {
                        dir: Direction::Top,
                    }
                    .into(),
                    (0.0, Unit::Em).into(),
                )
                    .into(),
                (
                    Padding {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    (0.0, Unit::Em).into(),
                )
                    .into(),
            ],
        };

        let img_date = CssBlock {
            selector: TopSelector::Class("img_date".to_owned()).into(),
            decls: vec![(Property::Float, Keyword::Left.into()).into()],
        };

        let img_nr = CssBlock {
            selector: TopSelector::Class("img_nr".to_owned()).into(),
            decls: vec![(Property::Float, Keyword::Right.into()).into()],
        };

        let left_arrow = CssBlock {
            selector: TopSelector::Class("left_arrow".to_owned()).into(),
            decls: vec![
                (Property::Float, Keyword::Left.into()).into(),
                (
                    Padding {
                        dir: Direction::Left,
                    }
                    .into(),
                    (0.2, Unit::Em).into(),
                )
                    .into(),
                (
                    Margin {
                        dir: Direction::Right,
                    }
                    .into(),
                    (0.2, Unit::Em).into(),
                )
                    .into(),
                (Property::Cursor, Keyword::Pointer.into()).into(),
                (Font::Size.into(), (24.0, Unit::Pt).into()).into(),
            ],
        };

        let right_arrow = CssBlock {
            selector: TopSelector::Class("right_arrow".to_owned()).into(),
            decls: vec![
                (Property::Float, Keyword::Right.into()).into(),
                (
                    Padding {
                        dir: Direction::Right,
                    }
                    .into(),
                    (0.2, Unit::Em).into(),
                )
                    .into(),
                (
                    Margin {
                        dir: Direction::Left,
                    }
                    .into(),
                    (0.2, Unit::Em).into(),
                )
                    .into(),
                (Property::Cursor, Keyword::Pointer.into()).into(),
                (Font::Size.into(), (24.0, Unit::Pt).into()).into(),
            ],
        };

        CssDocument {
            decls: vec![
                plant_gallery,
                images_plant_container,
                images_plant,
                plant_image,
                img,
                h2,
                img_controls,
                img_date,
                img_nr,
                left_arrow,
                right_arrow,
            ],
        }
    }
}
