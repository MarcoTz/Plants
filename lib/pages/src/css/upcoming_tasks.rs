use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Direction, Padding, Property},
    selector::TopSelector,
    value::{keyword::Keyword, unit::Unit, Value},
    CssDocument,
};
pub struct UpcomingTasks {}

impl CssComponent for UpcomingTasks {
    fn render(&self) -> CssDocument {
        let task_block = CssBlock {
            selector: TopSelector::Class("task_block".to_owned()).into(),
            decls: vec![
                (Property::MaxWidth, Value::Measurement(10.0, Unit::Em)).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
            ],
        };

        let upcoming_task = CssBlock {
            selector: TopSelector::Class("upcoming_task".to_owned()).into(),
            decls: vec![
                (Property::FontSize, Value::Measurement(12.0, Unit::Pt)).into(),
                (Property::FontWeight, Keyword::Bold.into()).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Property::Width, Value::Measurement(100.0, Unit::Percent)).into(),
                (
                    Padding {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    Value::Measurement(0.5, Unit::Em),
                )
                    .into(),
            ],
        };
        CssDocument {
            decls: vec![task_block, upcoming_task],
        }
    }
}
