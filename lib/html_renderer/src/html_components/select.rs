use super::component::{HtmlComponent, Render};
use std::rc::Rc;

pub struct SelectOption {
    pub value: String,
    pub content: Rc<HtmlComponent>,
}
pub struct Select {
    pub id: Option<String>,
    pub on_change: Option<String>,
    pub options: Vec<SelectOption>,
}

impl Render for Select {
    fn render(&self) -> String {
        let id_str = match self.id.clone() {
            None => "".to_owned(),
            Some(id) => format!("id=\"{id}\""),
        };
        let change_str = match self.on_change.clone() {
            None => "".to_owned(),
            Some(change) => format!("onChange=\"{change}\""),
        };
        let mut options_str = "".to_owned();
        for option in self.options.iter() {
            options_str.push_str(&option.render());
        }
        format!("<select {id_str} {change_str}>{options_str}</select>")
    }
}
impl Render for SelectOption {
    fn render(&self) -> String {
        let content_str = self.content.render();
        let value_str = self.value.clone();
        format!("<option value=\"{value_str}\">{content_str}</option>")
    }
}

impl From<Select> for HtmlComponent {
    fn from(select: Select) -> HtmlComponent {
        HtmlComponent::Select(select)
    }
}
