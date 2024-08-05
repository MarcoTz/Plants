use super::attribute::Attribute;
use super::html_element::{HtmlElement, Render};

pub struct Canvas {
    pub attributes: Vec<Attribute>,
}

impl Render for Canvas {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        format!("<canvas {attr_str}></canvas>")
    }
}

impl From<Canvas> for HtmlElement {
    fn from(canvas: Canvas) -> HtmlElement {
        HtmlElement::Canvas(canvas)
    }
}
