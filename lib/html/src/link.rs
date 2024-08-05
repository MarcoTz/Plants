use super::{attribute::Attribute, html_element::HtmlElement, render::Render};
use std::rc::Rc;

pub struct Link {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

impl Render for Link {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        let cont_str = self.content.render();

        format!("<a {attr_str}>{cont_str}</a>")
    }
}

impl From<Link> for HtmlElement {
    fn from(lnk: Link) -> HtmlElement {
        HtmlElement::Link(lnk)
    }
}
