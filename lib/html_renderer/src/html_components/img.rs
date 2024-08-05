use super::{
    attribute::Attribute,
    component::{HtmlComponent, Render},
};
pub struct Img {
    pub attributes: Vec<Attribute>,
}

impl Render for Img {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        format!("<img {attr_str} />")
    }
}

impl From<Img> for HtmlComponent {
    fn from(img: Img) -> HtmlComponent {
        HtmlComponent::Img(img)
    }
}
