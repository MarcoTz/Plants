use super::html_element::Render;

pub struct Head {
    pub title: String,
}

impl Render for Head {
    fn render(&self) -> String {
        let title = self.title.clone();
        format!("<head><title>{title}</title></head>")
    }
}
