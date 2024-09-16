use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Border, Color, Direction, Flex, Font, Margin, Padding, Property, Size},
    selector::TopSelector,
    value::{Keyword, Unit, Value},
    CssDocument,
};
use log;

pub struct PlantSearch {}

impl CssComponent for PlantSearch {
    fn render(&self) -> CssDocument {
        log::info!("Loading Plant Search CSS");
        let plant_search = CssBlock {
            selector: TopSelector::Id("plant_search".to_owned()).into(),
            decls: vec![
                (
                    Color::Background.into(),
                    Value::Var("bg-color-even".to_owned()),
                )
                    .into(),
                (Border::Radius.into(), (1.0, Unit::Em).into()).into(),
                (
                    Padding {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    (1.0, Unit::Em).into(),
                )
                    .into(),
                (
                    Padding {
                        dir: Direction::Top,
                    }
                    .into(),
                    (0.0, Unit::Em).into(),
                )
                    .into(),
                (Size::Width.into(), (90.0, Unit::Percent).into()).into(),
                (
                    Margin {
                        dir: Direction::All,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
                (Flex::Gap.into(), (0.5, Unit::Em).into()).into(),
            ],
        };

        let search_header = CssBlock {
            selector: TopSelector::Class("search_header".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Font::Weight.into(), Keyword::Bold.into()).into(),
                (Font::Size.into(), (14.0, Unit::Pt).into()).into(),
                (
                    Padding {
                        dir: Direction::Top,
                    }
                    .into(),
                    (0.5, Unit::Em).into(),
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

        CssDocument {
            decls: vec![plant_search, search_header],
        }
    }
}

#[cfg(test)]
mod plant_search_test {
    use super::{CssComponent, PlantSearch};
    use html::css::{
        block::CssBlock,
        property::{Border, Color, Direction, Flex, Font, Margin, Padding, Property, Size},
        selector::TopSelector,
        value::{Keyword, Unit, Value},
        CssDocument,
    };
    #[test]
    fn render_plant_search() {
        let result = PlantSearch {}.render();
        let expected = CssDocument {
            decls: vec![
                CssBlock {
                    selector: TopSelector::Id("plant_search".to_owned()).into(),
                    decls: vec![
                        (
                            Color::Background.into(),
                            Value::Var("bg-color-even".to_owned()),
                        )
                            .into(),
                        (Border::Radius.into(), (1.0, Unit::Em).into()).into(),
                        (
                            Padding {
                                dir: Direction::Bottom,
                            }
                            .into(),
                            (1.0, Unit::Em).into(),
                        )
                            .into(),
                        (
                            Padding {
                                dir: Direction::Top,
                            }
                            .into(),
                            (0.0, Unit::Em).into(),
                        )
                            .into(),
                        (Size::Width.into(), (90.0, Unit::Percent).into()).into(),
                        (
                            Margin {
                                dir: Direction::All,
                            }
                            .into(),
                            Keyword::Auto.into(),
                        )
                            .into(),
                        (Flex::Gap.into(), (0.5, Unit::Em).into()).into(),
                    ],
                },
                CssBlock {
                    selector: TopSelector::Class("search_header".to_owned()).into(),
                    decls: vec![
                        (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
                        (Property::TextAlign, Keyword::Center.into()).into(),
                        (Font::Weight.into(), Keyword::Bold.into()).into(),
                        (Font::Size.into(), (14.0, Unit::Pt).into()).into(),
                        (
                            Padding {
                                dir: Direction::Top,
                            }
                            .into(),
                            (0.5, Unit::Em).into(),
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
                        (
                            Margin {
                                dir: Direction::All,
                            }
                            .into(),
                            Keyword::Auto.into(),
                        )
                            .into(),
                    ],
                },
            ],
        };
        assert_eq!(result, expected)
    }
}
