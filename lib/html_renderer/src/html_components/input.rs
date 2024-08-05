use super::{
    attribute::Attribute,
    component::{HtmlComponent, Render},
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
impl From<Input> for HtmlComponent {
    fn from(input: Input) -> HtmlComponent {
        HtmlComponent::Input(input)
    }
}
