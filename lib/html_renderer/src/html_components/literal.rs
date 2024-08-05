use super::component::{HtmlComponent, Render};
pub struct Literal {
    content: String,
}

impl Render for Literal {
    fn render(&self) -> String {
        self.content.clone()
    }
}
impl From<Literal> for HtmlComponent {
    fn from(lit: Literal) -> HtmlComponent {
        HtmlComponent::Literal(lit)
    }
}
impl From<String> for Literal {
    fn from(s: String) -> Literal {
        Literal { content: s }
    }
}
