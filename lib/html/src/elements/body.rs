use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

pub struct Body {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

impl From<Body> for HtmlElement {
    fn from(bd: Body) -> HtmlElement {
        HtmlElement::Body(bd)
    }
}

impl From<HtmlElement> for Body {
    fn from(elem: HtmlElement) -> Body {
        Body {
            attributes: vec![],
            content: Rc::new(elem),
        }
    }
}

impl Render for Body {
    fn render(&self) -> String {
        let content_str = self.content.render().replace('\n', "\n\t");
        let attr_str = self.attributes.render().replace('\n', " ");
        format!("<body {attr_str}>\n\t{content_str}\n</body>")
    }
}
