use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Direction, Margin, Padding, Property},
    selector::TopSelector,
    value::{keyword::Keyword, unit::Unit, Value},
    CssDocument,
};

pub struct Gallery {}

impl CssComponent for Gallery {
    fn render(&self) -> CssDocument {
        let plant_gallery = CssBlock {
            selector: TopSelector::Id("plant_gallery".to_owned()).into(),
            decls: vec![
                (Property::Width, Value::Measurement(95.0, Unit::Percent)).into(),
                (
                    Padding {
                        dir: Direction::All,
                    }
                    .into(),
                    Value::Measurement(0.0, Unit::Em),
                )
                    .into(),
            ],
        };

        let images_plant_container = CssBlock {
            selector: TopSelector::Class("images_plant_container".to_owned()).into(),
            decls: vec![
                (Property::Width, Value::Measurement(19.0, Unit::Percent)).into(),
                (Property::FlexDirection, Keyword::Column.into()).into(),
                (
                    Padding {
                        dir: Direction::All,
                    }
                    .into(),
                    Value::Measurement(0.0, Unit::Em),
                )
                    .into(),
            ],
        };

        let images_plant = CssBlock {
            selector: TopSelector::Class("images_plant".to_owned()).into(),
            decls: vec![
                (Property::Width, Value::Measurement(90.0, Unit::Percent)).into(),
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
                    Value::Measurement(0.0, Unit::Em),
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
                (Property::Width, Value::Measurement(100.0, Unit::Percent)).into(),
                (Property::Height, Value::Measurement(100.0, Unit::Percent)).into(),
            ],
        };

        let h2 = CssBlock {
            selector: TopSelector::Tag("h2".to_owned()).into(),
            decls: vec![(
                Margin {
                    dir: Direction::All,
                }
                .into(),
                Value::Measurement(0.0, Unit::Em),
            )
                .into()],
        };

        let img_controls = CssBlock {
            selector: TopSelector::Class("img_controls".to_owned()).into(),
            decls: vec![
                (Property::AlignSelf, Keyword::FlexEnd.into()).into(),
                (Property::Width, Value::Measurement(100.0, Unit::Percent)).into(),
                (
                    Padding {
                        dir: Direction::Top,
                    }
                    .into(),
                    Value::Measurement(0.0, Unit::Em),
                )
                    .into(),
                (
                    Padding {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    Value::Measurement(0.0, Unit::Em),
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
                    Value::Measurement(0.2, Unit::Em),
                )
                    .into(),
                (
                    Margin {
                        dir: Direction::Right,
                    }
                    .into(),
                    Value::Measurement(0.2, Unit::Em),
                )
                    .into(),
                (Property::Cursor, Keyword::Pointer.into()).into(),
                (Property::FontSize, Value::Measurement(24.0, Unit::Pt)).into(),
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
                    Value::Measurement(0.2, Unit::Em),
                )
                    .into(),
                (
                    Margin {
                        dir: Direction::Left,
                    }
                    .into(),
                    Value::Measurement(0.2, Unit::Em),
                )
                    .into(),
                (Property::Cursor, Keyword::Pointer.into()).into(),
                (Property::FontSize, Value::Measurement(24.0, Unit::Pt)).into(),
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
