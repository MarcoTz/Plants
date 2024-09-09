use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct A {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

impl Render for A {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace('\n', " ");
        let cont_str = self.content.render();

        format!("<a {attr_str}>{cont_str}</a>")
    }
}

impl From<A> for HtmlElement {
    fn from(a: A) -> HtmlElement {
        HtmlElement::A(a)
    }
}

#[cfg(test)]
mod a_tests {

    use super::{Attribute, HtmlElement, Render, A};
    use std::rc::Rc;

    fn example_a() -> A {
        A {
            attributes: vec![Attribute::Href("link.html".to_owned())],
            content: Rc::new("a link".to_owned().into()),
        }
    }
    #[test]
    fn render_a() {
        let result = example_a().render();
        let expected = "<a href=\"link.html\">a link</a>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <A as Into<HtmlElement>>::into(example_a()).render();
        let expected = example_a().render();
        assert_eq!(result, expected)
    }
}
