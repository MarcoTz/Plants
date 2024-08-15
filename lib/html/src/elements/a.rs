use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

pub struct A {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

impl Render for A {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace('\n', " ");
        let cont_str = self.content.render();

        format!("<a {attr_str}>{cont_str}</a>")
    }
}

impl From<A> for HtmlElement {
    fn from(a: A) -> HtmlElement {
        HtmlElement::A(a)
    }
}
