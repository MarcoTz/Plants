use super::{
    attribute::Attribute,
    component::{HtmlComponent, Render},
};
use std::rc::Rc;

pub struct Div {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlComponent>,
}

impl From<Div> for HtmlComponent {
    fn from(dv: Div) -> HtmlComponent {
        HtmlComponent::Div(dv)
    }
}

impl Render for Div {
    fn render(&self) -> String {
        let content_str = self.content.render();
        let attr_str = self.attributes.render();
        format!("<div {attr_str}>{content_str}</div>")
    }
}
