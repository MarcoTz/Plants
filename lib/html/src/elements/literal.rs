use super::HtmlElement;
use crate::render::Render;

pub struct Literal {
    content: String,
}

impl Render for Literal {
    fn render(&self) -> String {
        self.content.clone()
    }
}
impl From<Literal> for HtmlElement {
    fn from(lit: Literal) -> HtmlElement {
        HtmlElement::Literal(lit)
    }
}
impl From<String> for Literal {
    fn from(s: String) -> Literal {
        Literal { content: s }
    }
}
