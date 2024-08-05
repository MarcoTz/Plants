use super::{
    attribute::Attribute,
    html_element::{HtmlElement, Render},
};
pub struct Input {
    pub attributes: Vec<Attribute>,
}

impl Render for Input {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        format!("<input {attr_str}/>")
    }
}
impl From<Input> for HtmlElement {
    fn from(input: Input) -> HtmlElement {
        HtmlElement::Input(input)
    }
}
