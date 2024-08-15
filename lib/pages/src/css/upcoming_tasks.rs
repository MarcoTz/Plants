use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Direction, Padding, Property},
    selector::TopSelector,
    value::{keyword::Keyword, unit::Unit},
    CssDocument,
};
pub struct UpcomingTasks {}

impl CssComponent for UpcomingTasks {
    fn render(&self) -> CssDocument {
        let task_block = CssBlock {
            selector: TopSelector::Class("task_block".to_owned()).into(),
            decls: vec![
                (Property::MaxWidth, (10.0, Unit::Em).into()).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
            ],
        };

        let upcoming_task = CssBlock {
            selector: TopSelector::Class("upcoming_task".to_owned()).into(),
            decls: vec![
                (Property::FontSize, (12.0, Unit::Pt).into()).into(),
                (Property::FontWeight, Keyword::Bold.into()).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Property::Width, (100.0, Unit::Percent).into()).into(),
                (
                    Padding {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    (0.5, Unit::Em).into(),
                )
                    .into(),
            ],
        };
        CssDocument {
            decls: vec![task_block, upcoming_task],
        }
    }
}
