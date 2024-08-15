use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

pub struct SelectOption {
    pub value: String,
    pub content: Rc<HtmlElement>,
}
pub struct Select {
    pub attributes: Vec<Attribute>,
    pub options: Vec<SelectOption>,
}

impl Render for Select {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace("\n", " ");
        let mut options_str = "".to_owned();
        for option in self.options.iter() {
            options_str.push_str(&option.render());
        }
        format!("<select {attr_str}>{options_str}</select>")
    }
}
impl Render for SelectOption {
    fn render(&self) -> String {
        let content_str = self.content.render();
        let value_str = self.value.clone();
        format!("<option value=\"{value_str}\">{content_str}</option>")
    }
}

impl From<Select> for HtmlElement {
    fn from(select: Select) -> HtmlElement {
        HtmlElement::Select(select)
    }
}
