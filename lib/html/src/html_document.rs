use super::{
    elements::{Body, Head},
    render::Render,
};

#[derive(Debug, PartialEq)]
pub struct HtmlDocument {
    pub head: Head,
    pub body: Body,
}

impl Render for HtmlDocument {
    fn render(&self) -> String {
        let head_str = self.head.render();
        let body_str = self.body.render();
        format!("<!doctype html>\n<html>\n\t{head_str}\n\t{body_str}</html>")
    }
}

#[cfg(test)]
mod document_tests {
    use super::{Body, Head, HtmlDocument, Render};
    use crate::{
        attribute::Attribute,
        css::CssDocument,
        elements::{Div, Style},
    };
    use std::rc::Rc;

    fn example_document1() -> HtmlDocument {
        HtmlDocument {
            head: Head {
                title: "example document".to_owned(),
                content: Rc::new(vec![].into()),
            },
            body: Body {
                attributes: vec![Attribute::Id("body".to_owned())],
                content: Rc::new("body".to_owned().into()),
            },
        }
    }

    fn example_document2() -> HtmlDocument {
        HtmlDocument {
            head: Head {
                title: "example document 2".to_owned(),
                content: Rc::new(
                    Style {
                        style: CssDocument { decls: vec![] },
                    }
                    .into(),
                ),
            },
            body: Body {
                attributes: vec![],
                content: Rc::new(
                    Div {
                        attributes: vec![],
                        content: Rc::new("hello".to_owned().into()),
                    }
                    .into(),
                ),
            },
        }
    }
    #[test]
    fn render_document1() {
        let result = example_document1().render();
        let expected = "<!doctype html>\n<html>\n\t<head>\n\t<title>\n\t\texample document\n\t</title>\n\t\n</head>\n\t<body id=\"body\">\n\tbody\n</body></html>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_document2() {
        let result = example_document2().render();
        let expected = "<!doctype html>\n<html>\n\t<head>\n\t<title>\n\t\texample document 2\n\t</title>\n\t<style>\n\t\t\n\t</style>\n</head>\n\t<body >\n\t<div >\n\t\thello\n\t</div>\n</body></html>";
        assert_eq!(result, expected)
    }
}
