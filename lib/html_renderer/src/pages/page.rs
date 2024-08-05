use super::super::html_components::component::{Html, HtmlComponent};

pub trait Page {
    fn render(&self) -> Html;
}

pub trait PageComponent {
    fn render(&self) -> HtmlComponent;
}
