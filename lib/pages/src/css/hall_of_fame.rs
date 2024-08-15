use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Direction, Margin, Property},
    selector::TopSelector,
    value::{keyword::Keyword, unit::Unit},
    CssDocument,
};

pub struct HallOfFame {}

impl CssComponent for HallOfFame {
    fn render(&self) -> CssDocument {
        let hall_of_fame = CssBlock {
            selector: TopSelector::Id("hall_of_fame".to_owned()).into(),
            decls: vec![
                (Property::Width, (95.0, Unit::Percent).into()).into(),
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
