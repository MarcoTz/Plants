use super::super::html::{html_document::HtmlDocument, html_element::HtmlElement};

pub trait Page {
    fn render(&self) -> HtmlDocument;
}

pub trait PageComponent {
    fn render(&self) -> HtmlElement;
}
