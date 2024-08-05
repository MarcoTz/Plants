use super::component::{HtmlComponent, Render};
use std::rc::Rc;

pub struct Link {
    pub href: String,
    pub content: Rc<HtmlComponent>,
    pub class: Option<String>,
}

impl Render for Link {
    fn render(&self) -> String {
        let href_str = self.href.clone();
        let cont_str = self.content.render();
        let class_str = match self.class.clone() {
            None => "".to_owned(),
            Some(cl) => format!("class=\"{cl}\""),
        };
        format!("<a {class_str} href=\"{href_str}\">{cont_str}</a>")
    }
}

impl From<Link> for HtmlComponent {
    fn from(lnk: Link) -> HtmlComponent {
        HtmlComponent::Link(lnk)
    }
}
