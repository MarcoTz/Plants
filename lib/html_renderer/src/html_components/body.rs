use super::component::{HtmlComponent, Render};
use std::rc::Rc;

pub struct Body {
    pub content: Rc<HtmlComponent>,
}

impl From<Body> for HtmlComponent {
    fn from(bd: Body) -> HtmlComponent {
        HtmlComponent::Body(bd)
    }
}
impl Render for Body {
    fn render(&self) -> String {
        let content_str = self.content.render();
        format!("<body>{content_str}</body>")
    }
}
