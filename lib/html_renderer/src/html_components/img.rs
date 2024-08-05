use super::component::{HtmlComponent, Render};
pub struct Img {
    pub src: String,
}

impl Render for Img {
    fn render(&self) -> String {
        let src_str = self.src.clone();
        format!("<img src=\"{src_str}\"/>")
    }
}

impl From<Img> for HtmlComponent {
    fn from(img: Img) -> HtmlComponent {
        HtmlComponent::Img(img)
    }
}
