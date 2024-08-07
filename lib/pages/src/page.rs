use html::html_document::HtmlDocument;

pub trait Page {
    fn render(&self, date_format: &str) -> HtmlDocument;
}
