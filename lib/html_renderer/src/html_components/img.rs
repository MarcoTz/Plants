use super::component::{HtmlComponent, Render};
pub struct Img {
    pub src: String,
    pub id: Option<String>,
    pub style: Option<String>,
}

impl Render for Img {
    fn render(&self) -> String {
        let src_str = self.src.clone();
        let id_str = match self.id.clone() {
            None => "".to_owned(),
            Some(id) => format!("id=\"{id}\""),
        };
        let style_str = match self.style.clone() {
            None => "".to_owned(),
            Some(style) => format!("style=\"{style}\""),
        };
        format!("<img {id_str} {style_str} src=\"{src_str}\"/>")
    }
}

impl From<Img> for HtmlComponent {
    fn from(img: Img) -> HtmlComponent {
        HtmlComponent::Img(img)
    }
}
