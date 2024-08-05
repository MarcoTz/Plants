use super::component::{HtmlComponent, Render};
use std::rc::Rc;

pub struct Div {
    pub class: Option<String>,
    pub id: Option<String>,
    pub content: Rc<HtmlComponent>,
}

impl From<Div> for HtmlComponent {
    fn from(dv: Div) -> HtmlComponent {
        HtmlComponent::Div(dv)
    }
}

impl Render for Div {
    fn render(&self) -> String {
        let content_str = self.content.render();
        let class_str = match self.class.clone() {
            None => "".to_owned(),
            Some(cl) => format!("class=\"{cl}\""),
        };
        let id_str = match self.id.clone() {
            None => "".to_owned(),
            Some(id) => format!("id=\"{id}\""),
        };
        format!("<div {class_str} {id_str}>{content_str}</div>")
    }
}
