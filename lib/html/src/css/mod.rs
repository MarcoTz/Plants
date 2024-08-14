pub mod block;
pub mod declaration;
pub mod property;
pub mod selector;
pub mod value;

use crate::{
    elements::{style::Style, HtmlElement},
    render::Render,
};
use block::CssBlock;

#[derive(Clone)]
pub struct CssDocument {
    pub decls: Vec<CssBlock>,
}

impl Render for CssDocument {
    fn render(&self) -> String {
        self.decls.render()
    }
}

impl From<CssDocument> for HtmlElement {
    fn from(css: CssDocument) -> HtmlElement {
        HtmlElement::Style(Style { styles: css.decls })
    }
}
