use super::{attribute::Attribute, html_element::HtmlElement, render::Render};

pub struct Link {
    pub attributes: Vec<Attribute>,
}

impl Render for Link {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        format!("<link {attr_str}/>")
    }
}

impl From<Link> for HtmlElement {
    fn from(lnk: Link) -> HtmlElement {
        HtmlElement::Link(lnk)
    }
}
