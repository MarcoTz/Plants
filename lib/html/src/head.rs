use super::html_element::HtmlElement;
use super::render::Render;
use std::rc::Rc;

pub struct Head {
    pub title: String,
    pub content: Rc<HtmlElement>,
}

impl From<Head> for HtmlElement {
    fn from(head: Head) -> HtmlElement {
        HtmlElement::Head(head)
    }
}

impl Render for Head {
    fn render(&self) -> String {
        let title = self.title.clone();
        let content_str = self.content.render();
        format!("<head><title>{title}</title>{content_str}</head>")
    }
}
