use super::{attribute::Attribute, html_element::HtmlElement, render::Render};

pub struct Script {
    pub attributes: Vec<Attribute>,
    pub content: String,
}

impl Render for Script {
    fn render(&self) -> String {
        let content_str = self.content.clone();
        let attr_str = self.attributes.render();
        format!("<script {attr_str} >{content_str}</script>")
    }
}

impl From<Script> for HtmlElement {
    fn from(script: Script) -> HtmlElement {
        HtmlElement::Script(script)
    }
}
