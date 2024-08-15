use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Border, Color, Direction, Flex, Font, Margin, Padding, Property, Size},
    selector::TopSelector,
    value::{keyword::Keyword, unit::Unit, Value},
    CssDocument,
};

pub struct PlantDetails {}

impl CssComponent for PlantDetails {
    fn render(&self) -> CssDocument {
        let plant_content = CssBlock {
            selector: TopSelector::Id("plant_content".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (90.0, Unit::Percent).into()).into(),
                (Property::Display, Keyword::FlowRoot.into()).into(),
            ],
        };

        let images_plant_container = CssBlock {
            selector: TopSelector::Class("images_plant_container".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (20.0, Unit::Percent).into()).into(),
                (Property::Float, Keyword::Left.into()).into(),
                (
                    Margin {
                        dir: Direction::Left,
                    }
                    .into(),
                    (5.0, Unit::Percent).into(),
                )
                    .into(),
                (
                    Color::Background.into(),
                    Value::Var("bg-color-even".to_owned()),
                )
                    .into(),
                (Border::Radius.into(), (1.0, Unit::Em).into()).into(),
            ],
        };

        let plant_info = CssBlock {
            selector: TopSelector::Id("plant_info".to_owned()).into(),
            decls: vec![(Size::Width.into(), (70.0, Unit::Percent).into()).into()],
        };

        let status_item = CssBlock {
            selector: TopSelector::Class("status_item".to_owned()).into(),
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
                (
                    Padding {
                        dir: Direction::All,
                    }
                    .into(),
                    (0.5, Unit::Em).into(),
                )
                    .into(),
            ],
        };

        let growth_log_container = CssBlock {
            selector: TopSelector::Id("plant_growth_log_container".to_owned()).into(),
            decls: vec![(Size::Width.into(), (100.0, Unit::Percent).into()).into()],
        };

        let plant_graph = CssBlock {
            selector: TopSelector::Class("plant_graph".to_owned()).into(),
            decls: vec![(Size::Width.into(), (45.0, Unit::Percent).into()).into()],
        };

        let activities_container = CssBlock {
            selector: TopSelector::Id("plant_activities_container".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (95.0, Unit::Percent).into()).into(),
                (Flex::FlexWrap.into(), Keyword::NoWrap.into()).into(),
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

        let health = CssBlock {
            selector: TopSelector::Class("health".to_owned()).into(),
            decls: vec![
                (
                    Property::Var("circle-radius".to_owned()),
                    (2.0, Unit::Em).into(),
                )
                    .into(),
                (Border::Radius.into(), (50.0, Unit::Percent).into()).into(),
                (Size::Width.into(), Value::Var("circle-radius".to_owned())).into(),
                (Size::Height.into(), Value::Var("circle-radius".to_owned())).into(),
                (Property::LineHeight, Value::Var("circle-radius".to_owned())).into(),
                (Property::TextAlign, Keyword::Center.into()).into(),
                (Color::Color.into(), Value::Var("fg-color-dark".to_owned())).into(),
                (Font::Size.into(), (18.0, Unit::Pt).into()).into(),
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

        let health0 = CssBlock {
            selector: TopSelector::Class("health0".to_owned()).into(),
            decls: vec![(
                Color::Background.into(),
                Value::Var("health-color0".to_owned()),
            )
                .into()],
        };
        let health1 = CssBlock {
            selector: TopSelector::Class("health1".to_owned()).into(),
            decls: vec![(
                Color::Background.into(),
                Value::Var("health-color1".to_owned()),
            )
                .into()],
        };
        let health2 = CssBlock {
            selector: TopSelector::Class("health2".to_owned()).into(),
            decls: vec![(
                Color::Background.into(),
                Value::Var("health-color2".to_owned()),
            )
                .into()],
        };
        let health3 = CssBlock {
            selector: TopSelector::Class("health3".to_owned()).into(),
            decls: vec![(
                Color::Background.into(),
                Value::Var("health-color3".to_owned()),
            )
                .into()],
        };
        let health4 = CssBlock {
            selector: TopSelector::Class("health4".to_owned()).into(),
            decls: vec![(
                Color::Background.into(),
                Value::Var("health-color4".to_owned()),
            )
                .into()],
        };

        let health5 = CssBlock {
            selector: TopSelector::Class("health5".to_owned()).into(),
            decls: vec![(
                Color::Background.into(),
                Value::Var("health-color5".to_owned()),
            )
                .into()],
        };

        CssDocument {
            decls: vec![
                plant_content,
                images_plant_container,
                plant_info,
                status_item,
                growth_log_container,
                plant_graph,
                activities_container,
                health,
                health0,
                health1,
                health2,
                health3,
                health4,
                health5,
            ],
        }
    }
}
