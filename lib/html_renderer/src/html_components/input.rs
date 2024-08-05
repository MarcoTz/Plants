use super::component::{HtmlComponent, Render};
pub struct Input {
    pub id: Option<String>,
    pub keyup: Option<String>,
    pub ty: Option<String>,
}

impl Render for Input {
    fn render(&self) -> String {
        let id_str = match self.id.clone() {
            None => "".to_owned(),
            Some(id) => format!("id=\"{id}\""),
        };
        let keyup_str = match self.keyup.clone() {
            None => "".to_owned(),
            Some(keyup) => format!("onKeyUp=\"{keyup}\""),
        };
        let ty_str = match self.ty.clone() {
            None => "".to_owned(),
            Some(ty) => format!("type=\"{ty}\""),
        };
        format!("<input {id_str} {keyup_str} {ty_str}/>")
    }
}
impl From<Input> for HtmlComponent {
    fn from(input: Input) -> HtmlComponent {
        HtmlComponent::Input(input)
    }
}
