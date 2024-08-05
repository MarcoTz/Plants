use super::{
    attribute::Attribute,
    component::{HtmlComponent, Render},
};
use std::rc::Rc;
pub struct Figure {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlComponent>,
    pub caption: Rc<HtmlComponent>,
}

impl Render for Figure {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        let contents_str = self.content.render();
        let caption_str = self.caption.render();
        format!("<figure {attr_str}>{contents_str}<figcaption>{caption_str}</figcaption></figure>")
    }
}

impl From<Figure> for HtmlComponent {
    fn from(fig: Figure) -> HtmlComponent {
        HtmlComponent::Figure(fig)
    }
}
