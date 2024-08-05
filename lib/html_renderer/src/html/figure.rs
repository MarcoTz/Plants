use super::{
    attribute::Attribute,
    html_element::{HtmlElement, Render},
};
use std::rc::Rc;
pub struct Figure {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
    pub caption: Rc<HtmlElement>,
}

impl Render for Figure {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        let contents_str = self.content.render();
        let caption_str = self.caption.render();
        format!("<figure {attr_str}>{contents_str}<figcaption>{caption_str}</figcaption></figure>")
    }
}

impl From<Figure> for HtmlElement {
    fn from(fig: Figure) -> HtmlElement {
        HtmlElement::Figure(fig)
    }
}
