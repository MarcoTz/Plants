use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    declaration::Declaration,
    property::{Direction, Margin, Padding, Property},
    selector::TopSelector,
    value::{keyword::Keyword, unit::Unit, Value},
    CssDocument,
};

pub struct PlantList {}

impl CssComponent for PlantList {
    fn render(&self) -> CssDocument {
        let plant_list_item = CssBlock {
            selector: TopSelector::Class("plant_list_item".to_owned()).into(),
            decls: vec![
                (Property::Height, Value::Measurement(9.0, Unit::Em)).into(),
                (Property::Width, Value::Measurement(10.0, Unit::Em)).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Property::Overflow, Keyword::Hidden.into()).into(),
            ],
        };

        let plant_preview = CssBlock {
            selector: TopSelector::Class("plant_preview".to_owned()).into(),
            decls: vec![
                (Property::Height, Value::Measurement(40.0, Unit::Percent)).into(),
                (
                    Margin {
                        dir: Direction::Top,
                    }
                    .into(),
                    Value::Measurement(0.5, Unit::Em),
                )
                    .into(),
            ],
        };

        let temps = CssBlock {
            selector: TopSelector::Multiple(vec![
                TopSelector::Class("temp_max".to_owned()),
                TopSelector::Class("temp_min".to_owned()),
            ])
            .into(),
            decls: vec![(Property::Display, Keyword::Non.into()).into()],
        };

        let location_header = CssBlock {
            selector: TopSelector::Class("location_header".to_owned()).into(),
            decls: vec![
                Declaration {
                    property: Property::Background,
                    value: Keyword::Transparent.into(),
                    important: true,
                },
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

        CssDocument {
            decls: vec![plant_list_item, plant_preview, temps, location_header],
        }
    }
}

/**/
