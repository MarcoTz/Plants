use super::{html_element::HtmlElement, render::Render};

pub struct Script {
    pub content: String,
}

impl Render for Script {
    fn render(&self) -> String {
        let content_str = self.content.clone();
        format!("<script>{content_str}</script>")
    }
}

impl From<Script> for HtmlElement {
    fn from(script: Script) -> HtmlElement {
        HtmlElement::Script(script)
    }
}
