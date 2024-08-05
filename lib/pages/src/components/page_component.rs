use html::html_element::HtmlElement;

pub trait PageComponent {
    fn render(&self) -> HtmlElement;
}
