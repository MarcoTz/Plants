use crate::{css::block::CssBlock, render::Render};

pub struct Style {
    pub styles: Vec<CssBlock>,
}

impl Render for Style {
    fn render(&self) -> String {
        let blocks_str = self.styles.render();
        format!("<style>{blocks_str}</style>")
    }
}
