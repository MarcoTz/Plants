use super::super::html::html_element::{Html, HtmlElement};

pub trait Page {
    fn render(&self) -> Html;
}

pub trait PageComponent {
    fn render(&self) -> HtmlElement;
}
