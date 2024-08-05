use super::attribute::Attribute;
use super::component::{HtmlComponent, Render};

pub struct Canvas {
    pub attributes: Vec<Attribute>,
}

impl Render for Canvas {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        format!("<canvas {attr_str}></canvas>")
    }
}

impl From<Canvas> for HtmlComponent {
    fn from(canvas: Canvas) -> HtmlComponent {
        HtmlComponent::Canvas(canvas)
    }
}
