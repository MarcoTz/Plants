use super::{html_element::HtmlElement, render::Render};
use std::rc::Rc;

pub struct Body {
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
            content: Rc::new(elem),
        }
    }
}

impl Render for Body {
    fn render(&self) -> String {
        let content_str = self.content.render();
        format!("<body>{content_str}</body>")
    }
}
