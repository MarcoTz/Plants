use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Div {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

impl From<Div> for HtmlElement {
    fn from(dv: Div) -> HtmlElement {
        HtmlElement::Div(dv)
    }
}

impl Render for Div {
    fn render(&self) -> String {
        let content_str = self.content.render().replace('\n', "\n\t");
        let attr_str = self.attributes.render().replace('\n', " ");
        format!("<div {attr_str}>\n\t{content_str}\n</div>")
    }
}

#[cfg(test)]
mod div_tests {

    use super::{Attribute, Div, HtmlElement, Render};
    use std::rc::Rc;

    fn example_div() -> Div {
        Div {
            attributes: vec![Attribute::Class(vec!["div".to_owned()])],
            content: Rc::new("a div".to_owned().into()),
        }
    }

    #[test]
    fn render_div() {
        let result = example_div().render();
        let expected = "<div class=\"div\">\n\ta div\n</div>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Div as Into<HtmlElement>>::into(example_div()).render();
        let expected = example_div().render();
        assert_eq!(result, expected)
    }
}
