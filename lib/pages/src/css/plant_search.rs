use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Direction, Margin, Padding, Property},
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
                (Property::BorderRadius, Value::Measurement(1.0, Unit::Em)).into(),
                (
                    Padding {
                        dir: Direction::Bottom,
                    }
                    .into(),
                    Value::Measurement(1.0, Unit::Em),
                )
                    .into(),
                (
                    Padding {
                        dir: Direction::Top,
                    }
                    .into(),
                    Value::Measurement(0.0, Unit::Em),
                )
                    .into(),
                (Property::Width, Value::Measurement(90.0, Unit::Percent)).into(),
                (
                    Margin {
                        dir: Direction::All,
                    }
                    .into(),
                    Keyword::Auto.into(),
                )
                    .into(),
                (Property::Gap, Value::Measurement(0.5, Unit::Em)).into(),
            ],
        };

        let search_header = CssBlock {
            selector: TopSelector::Class("search_header".to_owned()).into(),
            decls: vec![
                (Property::Width, Value::Measurement(100.0, Unit::Percent)).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Property::FontWeight, Keyword::Bold.into()).into(),
                (Property::FontSize, Value::Measurement(14.0, Unit::Pt)).into(),
                (
                    Padding {
                        dir: Direction::Top,
                    }
                    .into(),
                    Value::Measurement(0.5, Unit::Em),
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
