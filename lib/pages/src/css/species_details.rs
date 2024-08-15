use crate::page::CssComponent;
use html::css::{
    block::CssBlock,
    property::{Color, Direction, Margin, Size},
    selector::{Selector, SubSelector, TopSelector},
    value::{keyword::Keyword, unit::Unit},
    CssDocument,
};
use std::rc::Rc;

pub struct SpeciesDetails {}

impl CssComponent for SpeciesDetails {
    fn render(&self) -> CssDocument {
        let species_content = CssBlock {
            selector: TopSelector::Id("species_content".to_owned()).into(),
            decls: vec![
                (Size::Width.into(), (95.0, Unit::Percent).into()).into(),
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

        let species_details_gallery = CssBlock {
            selector: TopSelector::Id("species_details_gallery".to_owned()).into(),
            decls: vec![(Size::Width.into(), (95.0, Unit::Percent).into()).into()],
        };
        let species_gallery_header = CssBlock {
            selector: Selector {
                top: TopSelector::Id("species_details_gallery".to_owned()),
                sub: Some(SubSelector::ChildCombinator(Rc::new(
                    TopSelector::Tag("h2".to_owned()).into(),
                ))),
            },
            decls: vec![(Color::Background.into(), Keyword::Transparent.into()).into()],
        };
        CssDocument {
            decls: vec![
                species_content,
                species_details_gallery,
                species_gallery_header,
            ],
        }
    }
}
