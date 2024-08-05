use super::component::{HtmlComponent, Render};
use std::rc::Rc;
pub struct Figure {
    pub class: Option<String>,
    pub contents: Rc<HtmlComponent>,
    pub caption: Rc<HtmlComponent>,
}

impl Render for Figure {
    fn render(&self) -> String {
        let class_str = match self.class.clone() {
            None => "".to_owned(),
            Some(cl) => format!("class=\"{cl}\""),
        };
        let contents_str = self.contents.render();
        let caption_str = self.caption.render();
        format!("<figure class=\"{class_str}\">{contents_str}<figcaption>{caption_str}</figcaption></figure>")
    }
}

impl From<Figure> for HtmlComponent {
    fn from(fig: Figure) -> HtmlComponent {
        HtmlComponent::Figure(fig)
    }
}
