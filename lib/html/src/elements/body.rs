use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Body {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

impl From<Body> for HtmlElement {
    fn from(bd: Body) -> HtmlElement {
        HtmlElement::Body(bd)
    }
}

impl From<HtmlElement> for Body {
    fn from(elem: HtmlElement) -> Body {
        Body {
            attributes: vec![],
            content: Rc::new(elem),
        }
    }
}

impl Render for Body {
    fn render(&self) -> String {
        let content_str = self.content.render().replace('\n', "\n\t");
        let attr_str = self.attributes.render().replace('\n', " ");
        format!("<body {attr_str}>\n\t{content_str}\n</body>")
    }
}

#[cfg(test)]
mod body_tests {

    use super::{Attribute, Body, HtmlElement, Render};
    use std::rc::Rc;

    fn example_body() -> Body {
        Body {
            attributes: vec![Attribute::Id("body".to_owned())],
            content: Rc::new("a html body".to_owned().into()),
        }
    }

    #[test]
    fn render_body() {
        let result = example_body().render();
        let expected = "<body id=\"body\">\n\ta html body\n</body>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Body as Into<HtmlElement>>::into(example_body()).render();
        let expected = example_body().render();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_from() {
        let elem: HtmlElement = "an element".to_owned().into();
        let body: Body = elem.into();
        let result = body.render();
        let expected = "<body >\n\tan element\n</body>";
        assert_eq!(result, expected)
    }
}
