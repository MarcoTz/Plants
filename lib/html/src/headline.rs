use super::{attribute::Attribute, html_element::HtmlElement, render::Render};
use std::rc::Rc;

pub enum HeaderSize {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

pub struct Headline {
    pub size: HeaderSize,
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

impl From<Headline> for HtmlElement {
    fn from(hd: Headline) -> HtmlElement {
        HtmlElement::Headline(hd)
    }
}
impl Render for Headline {
    fn render(&self) -> String {
        let size_tag = self.size.render();
        let content_str = self.content.render();
        let attr_str = self.attributes.render();
        format!("<{size_tag} {attr_str}>{content_str}</{size_tag}>")
    }
}

impl Render for HeaderSize {
    fn render(&self) -> String {
        match self {
            HeaderSize::H1 => "h1".to_owned(),
            HeaderSize::H2 => "h2".to_owned(),
            HeaderSize::H3 => "h3".to_owned(),
            HeaderSize::H4 => "h4".to_owned(),
            HeaderSize::H5 => "h5".to_owned(),
            HeaderSize::H6 => "h6".to_owned(),
        }
    }
}