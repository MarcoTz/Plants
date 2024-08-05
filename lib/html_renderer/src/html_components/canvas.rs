use super::component::{HtmlComponent, Render};

pub struct Canvas {
    pub id: String,
}

impl Render for Canvas {
    fn render(&self) -> String {
        let id_str = self.id.clone();
        format!("<canvas id=\"{id_str}\"></canvas>")
    }
}

impl From<Canvas> for HtmlComponent {
    fn from(canvas: Canvas) -> HtmlComponent {
        HtmlComponent::Canvas(canvas)
    }
}
