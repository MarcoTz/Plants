use super::{attribute::Attribute, html_element::HtmlElement, render::Render};
pub struct Img {
    pub attributes: Vec<Attribute>,
}

impl Render for Img {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        format!("<img {attr_str} />")
    }
}

impl From<Img> for HtmlElement {
    fn from(img: Img) -> HtmlElement {
        HtmlElement::Img(img)
    }
}
