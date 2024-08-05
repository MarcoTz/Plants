use super::{
    attribute::Attribute,
    component::{HtmlComponent, Render},
};
use std::rc::Rc;

pub struct Link {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlComponent>,
}

impl Render for Link {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        let cont_str = self.content.render();

        format!("<a {attr_str}>{cont_str}</a>")
    }
}

impl From<Link> for HtmlComponent {
    fn from(lnk: Link) -> HtmlComponent {
        HtmlComponent::Link(lnk)
    }
}
