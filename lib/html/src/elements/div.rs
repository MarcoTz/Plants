use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

pub struct Div {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

impl From<Div> for HtmlElement {
    fn from(dv: Div) -> HtmlElement {
        HtmlElement::Div(dv)
    }
}

impl Render for Div {
    fn render(&self) -> String {
        let content_str = self.content.render();
        let attr_str = self.attributes.render();
        format!("<div {attr_str}>{content_str}</div>")
    }
}
