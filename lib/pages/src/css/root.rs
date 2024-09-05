use crate::page::CssComponent;
use html::css::{
    block::CssBlock, property::Property, selector::TopSelector, value::Color, CssDocument,
};

pub struct Root {}

impl CssComponent for Root {
    fn render(&self) -> CssDocument {
        log::info!("Loading root css");
        vec![CssBlock {
            selector: TopSelector::Pseudo("root".to_owned()).into(),
            decls: vec![
                (
                    Property::Var("bg-color".to_owned()),
                    Color::Rgb(34, 35, 39).into(),
                )
                    .into(),
                (
                    Property::Var("fg-color".to_owned()),
                    Color::Rgb(102, 204, 224).into(),
                )
                    .into(),
                (
                    Property::Var("fg-color-dark".to_owned()),
                    Color::Rgb(51, 102, 112).into(),
                )
                    .into(),
                (
                    Property::Var("link-color".to_owned()),
                    Color::Rgb(158, 208, 114).into(),
                )
                    .into(),
                (
                    Property::Var("link-color-visited".to_owned()),
                    Color::Rgb(114, 162, 120).into(),
                )
                    .into(),
                (
                    Property::Var("bg-color-odd".to_owned()),
                    Color::Rgb(42, 45, 55).into(),
                )
                    .into(),
                (
                    Property::Var("bg-color-even".to_owned()),
                    Color::Rgb(66, 86, 97).into(),
                )
                    .into(),
                (
                    Property::Var("bg-color-trans".to_owned()),
                    Color::Rgba(22, 23, 27, 0.8).into(),
                )
                    .into(),
                (
                    Property::Var("health-color0".to_owned()),
                    Color::Rgb(117, 118, 117).into(),
                )
                    .into(),
                (
                    Property::Var("health-color1".to_owned()),
                    Color::Rgb(214, 10, 14).into(),
                )
                    .into(),
                (
                    Property::Var("health-color2".to_owned()),
                    Color::Rgb(249, 123, 14).into(),
                )
                    .into(),
                (
                    Property::Var("health-color3".to_owned()),
                    Color::Rgb(217, 224, 8).into(),
                )
                    .into(),
                (
                    Property::Var("health-color4".to_owned()),
                    Color::Rgb(146, 229, 19).into(),
                )
                    .into(),
                (
                    Property::Var("health-color5".to_owned()),
                    Color::Rgb(51, 216, 24).into(),
                )
                    .into(),
            ],
        }]
        .into()
    }
}
