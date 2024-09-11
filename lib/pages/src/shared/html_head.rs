use crate::{css::PageCss, page::PageComponent};
use html::{
    attribute::Attribute,
    elements::{Head, HtmlElement, Script},
};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HtmlHead {
    pub title: String,
    pub styles: PageCss,
    pub scripts: Vec<String>,
    pub date_format: String,
}

impl From<&HtmlHead> for Head {
    fn from(hd: &HtmlHead) -> Head {
        let mut scripts = vec![];
        for script in hd.scripts.iter() {
            scripts.push(
                Script {
                    attributes: vec![Attribute::Src(script.clone())],
                    content: "".to_owned(),
                }
                .into(),
            )
        }

        Head {
            title: hd.title.clone(),
            content: Rc::new(vec![hd.styles.render(&hd.date_format), scripts.into()].into()),
        }
    }
}
impl PageComponent for HtmlHead {
    fn render(&self, _: &str) -> HtmlElement {
        Head::from(self).into()
    }
}

#[cfg(test)]
mod html_head_tests {

    use super::{HtmlHead, PageComponent, PageCss};
    use crate::test_common::DATE_FORMAT;
    use html::{
        attribute::Attribute,
        elements::{Head, Script},
    };
    use std::rc::Rc;

    fn example_html_head() -> HtmlHead {
        HtmlHead {
            title: "Testing".to_owned(),
            styles: PageCss::Index,
            scripts: vec!["js/main.js".to_owned(), "js/test.js".to_owned()],
            date_format: DATE_FORMAT.to_owned(),
        }
    }

    fn example_head() -> Head {
        Head {
            title: "Testing".to_owned(),
            content: Rc::new(
                vec![
                    PageCss::Index.render(DATE_FORMAT),
                    vec![
                        Script {
                            attributes: vec![Attribute::Src("js/main.js".to_owned())],
                            content: "".to_owned(),
                        }
                        .into(),
                        Script {
                            attributes: vec![Attribute::Src("js/test.js".to_owned())],
                            content: "".to_owned(),
                        }
                        .into(),
                    ]
                    .into(),
                ]
                .into(),
            ),
        }
    }
    #[test]
    fn head_into() {
        let result = <&HtmlHead as Into<Head>>::into(&example_html_head());
        let expected = example_head();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_head() {
        let result = example_html_head().render(DATE_FORMAT);
        let expected = example_head().into();
        assert_eq!(result, expected)
    }
}
