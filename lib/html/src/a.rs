use super::{attribute::Attribute, html_element::HtmlElement, render::Render};
use std::rc::Rc;

pub struct A {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

impl Render for A {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        let cont_str = self.content.render();

        format!("<a {attr_str}>{cont_str}</a>")
    }
}

impl From<A> for HtmlElement {
    fn from(a: A) -> HtmlElement {
        HtmlElement::A(a)
    }
}