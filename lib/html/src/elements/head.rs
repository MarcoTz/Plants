use super::HtmlElement;
use crate::render::Render;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Head {
    pub title: String,
    pub content: Rc<HtmlElement>,
}

impl From<Head> for HtmlElement {
    fn from(head: Head) -> HtmlElement {
        HtmlElement::Head(head)
    }
}

impl Render for Head {
    fn render(&self) -> String {
        let title = self.title.clone();
        let content_str = self.content.render().replace('\n', "\n\t");
        format!("<head>\n\t<title>\n\t\t{title}\n\t</title>\n\t{content_str}\n</head>")
    }
}

#[cfg(test)]
mod head_tests {
    use super::{Head, HtmlElement, Render};
    use std::rc::Rc;

    fn example_head() -> Head {
        Head {
            title: "a page".to_owned(),
            content: Rc::new("no other loads".to_owned().into()),
        }
    }

    #[test]
    fn render_head() {
        let result = example_head().render();
        let expected = "<head>\n\t<title>\n\t\ta page\n\t</title>\n\tno other loads\n</head>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Head as Into<HtmlElement>>::into(example_head()).render();
        let expected = example_head().render();
        assert_eq!(result, expected)
    }
}
