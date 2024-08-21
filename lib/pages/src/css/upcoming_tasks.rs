use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Direction, Font, Padding, Property, Size},
    selector::TopSelector,
    value::{Keyword, Unit},
    CssDocument,
};
pub struct UpcomingTasks {}

impl CssComponent for UpcomingTasks {
    fn render(&self) -> CssDocument {
        log::info!("Loading CSS for upcoming tasks");
        let task_block = CssBlock {
            selector: TopSelector::Class("task_block".to_owned()).into(),
            decls: vec![
                (Size::MaxWidth.into(), (10.0, Unit::Em).into()).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
            ],
        };

        let upcoming_task = CssBlock {
            selector: TopSelector::Class("upcoming_task".to_owned()).into(),
            decls: vec![
                (Font::Size.into(), (12.0, Unit::Pt).into()).into(),
                (Font::Weight.into(), Keyword::Bold.into()).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Size::Width.into(), (100.0, Unit::Percent).into()).into(),
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
