pub mod footer;
pub mod header;
pub mod main;

use crate::page::CssComponent;
use html::css::CssDocument;

use footer::Footer;
use header::Header;
use main::Main;

#[derive(Clone)]
pub enum DefinedDocument {
    Main,
    Header,
    Footer,
}

impl CssComponent for DefinedDocument {
    fn render(&self) -> CssDocument {
        match self {
            DefinedDocument::Main => Main {}.render(),
            DefinedDocument::Header => Header {}.render(),
            DefinedDocument::Footer => Footer {}.render(),
        }
    }
}
