use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

pub struct Figure {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
    pub caption: Rc<HtmlElement>,
}

impl Render for Figure {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace("\n", " ");
        let contents_str = self.content.render().replace("\n", "\n\t");
        let caption_str = self.caption.render().replace("\n", "\n\t\t");
        format!("<figure {attr_str}>\n\t{contents_str}\n\t<figcaption>\n\t\t{caption_str}\n\t</figcaption>\n</figure>")
    }
}

impl From<Figure> for HtmlElement {
    fn from(fig: Figure) -> HtmlElement {
        HtmlElement::Figure(fig)
    }
}
