use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};

pub struct Script {
    pub attributes: Vec<Attribute>,
    pub content: String,
}

impl Render for Script {
    fn render(&self) -> String {
        let content_str = self.content.clone().replace('\n', "\n\t");
        let attr_str = self.attributes.render().replace('\n', " ");
        format!("<script {attr_str} >\n\t{content_str}\n</script>")
    }
}

impl From<Script> for HtmlElement {
    fn from(script: Script) -> HtmlElement {
        HtmlElement::Script(script)
    }
}
