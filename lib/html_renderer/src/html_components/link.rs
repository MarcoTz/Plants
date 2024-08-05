use super::component::{HtmlComponent, Render};
use std::rc::Rc;

pub struct Link {
    pub href: String,
    pub contents: Rc<HtmlComponent>,
}

impl Render for Link {
    fn render(&self) -> String {
        let href_str = self.href.clone();
        let cont_str = self.contents.render();
        format!("<a href=\"{href_str}\">{cont_str}</a>")
    }
}

impl From<Link> for HtmlComponent {
    fn from(lnk: Link) -> HtmlComponent {
        HtmlComponent::Link(lnk)
    }
}
