use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Direction, Margin, Property, Size},
    selector::TopSelector,
    value::{Keyword, Unit},
    CssDocument,
};

pub struct HallOfFame {}

impl CssComponent for HallOfFame {
    fn render(&self) -> CssDocument {
        log::info!("Loading Hall of Fame css");
        let hall_of_fame = CssBlock {
            selector: TopSelector::Id("hall_of_fame".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (95.0, Unit::Percent).into()).into(),
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

        let hall_of_fame_item = CssBlock {
            selector: TopSelector::Class("hall_of_fame_item".to_owned()).into(),
            decls: vec![
                (
                    Margin {
                        dir: Direction::All,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
            ],
        };
        CssDocument {
            decls: vec![hall_of_fame, hall_of_fame_item],
        }
    }
}

#[cfg(test)]
mod hall_fo_fame_test {
    use super::{CssComponent, HallOfFame};
    use html::css::{
        block::CssBlock,
        property::{Direction, Margin, Property, Size},
        selector::TopSelector,
        value::{Keyword, Unit},
        CssDocument,
    };

    #[test]
    fn render_hall_of_fame() {
        let result = HallOfFame {}.render();
        let expected = CssDocument {
            decls: vec![
                CssBlock {
                    selector: TopSelector::Id("hall_of_fame".to_owned()).into(),
                    decls: vec![
                        (Size::Width.into(), (95.0, Unit::Percent).into()).into(),
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
                CssBlock {
                    selector: TopSelector::Class("hall_of_fame_item".to_owned()).into(),
                    decls: vec![
                        (
                            Margin {
                                dir: Direction::All,
                            }
                            .into(),
                            Keyword::Auto.into(),
                        )
                            .into(),
                        (Property::TextAlign, Keyword::Center.into()).into(),
                    ],
                },
            ],
        };
        assert_eq!(result, expected)
    }
}
