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
        let content_str = self.content.render().replace("\n", "\n\t");
        let attr_str = self.attributes.render().replace("\n", " ");
        format!("<div {attr_str}>\n\t{content_str}\n</div>")
    }
}
