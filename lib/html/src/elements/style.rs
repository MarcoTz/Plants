use crate::elements::HtmlElement;
use crate::{css::CssDocument, render::Render};

pub struct Style {
    pub styles: Vec<CssDocument>,
}

impl Render for Style {
    fn render(&self) -> String {
        let blocks_str = self.styles.render().replace("\n", "\n\t");
        format!("<style>\n\t{blocks_str}\n</style>")
    }
}

impl From<Style> for HtmlElement {
    fn from(style: Style) -> HtmlElement {
        HtmlElement::Style(style)
    }
}
