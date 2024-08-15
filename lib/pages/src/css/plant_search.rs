use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Border, Direction, Margin, Padding, Property},
    selector::TopSelector,
    value::{keyword::Keyword, unit::Unit, Value},
    CssDocument,
};

pub struct PlantSearch {}

impl CssComponent for PlantSearch {
    fn render(&self) -> CssDocument {
        let plant_search = CssBlock {
            selector: TopSelector::Id("plant_search".to_owned()).into(),
            decls: vec![
                (Property::Background, Value::Var("bg-color-even".to_owned())).into(),
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
                (Property::Width, (90.0, Unit::Percent).into()).into(),
                (
                    Margin {
                        dir: Direction::All,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
                (Property::Gap, (0.5, Unit::Em).into()).into(),
            ],
        };

        let search_header = CssBlock {
            selector: TopSelector::Class("search_header".to_owned()).into(),
            decls: vec![
                (Property::Width, (100.0, Unit::Percent).into()).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Property::FontWeight, Keyword::Bold.into()).into(),
                (Property::FontSize, (14.0, Unit::Pt).into()).into(),
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
