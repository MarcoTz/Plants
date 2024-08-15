use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    declaration::Declaration,
    property::{Color, Direction, Margin, Padding, Property, Size},
    selector::TopSelector,
    value::{keyword::Keyword, unit::Unit},
    CssDocument,
};

pub struct PlantList {}

impl CssComponent for PlantList {
    fn render(&self) -> CssDocument {
        let plant_list_item = CssBlock {
            selector: TopSelector::Class("plant_list_item".to_owned()).into(),
            decls: vec![
                (Size::Height.into(), (9.0, Unit::Em).into()).into(),
                (Size::Width.into(), (10.0, Unit::Em).into()).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Property::Overflow, Keyword::Hidden.into()).into(),
            ],
        };

        let plant_preview = CssBlock {
            selector: TopSelector::Class("plant_preview".to_owned()).into(),
            decls: vec![
                (Size::Height.into(), (40.0, Unit::Percent).into()).into(),
                (
                    Margin {
                        dir: Direction::Top,
                    }
                    .into(),
                    (0.5, Unit::Em).into(),
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
                    property: Color::Background.into(),
                    value: Keyword::Transparent.into(),
                    important: true,
                },
                (
                    Padding {
                        dir: Direction::All,
                    }
                    .into(),
                    (0.0, Unit::Em).into(),
                )
                    .into(),
            ],
        };

        CssDocument {
            decls: vec![plant_list_item, plant_preview, temps, location_header],
        }
    }
}
