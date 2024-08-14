use super::{declaration::Declaration, selector::Selector};
use crate::render::Render;

#[derive(Clone)]
pub struct CssBlock {
    pub selector: Selector,
    pub decls: Vec<Declaration>,
}

impl Render for CssBlock {
    fn render(&self) -> String {
        let selector_str = self.selector.render();
        let decls_str = self.decls.render();

        format!("{selector_str} {{ \n {decls_str}\n }}")
    }
}
