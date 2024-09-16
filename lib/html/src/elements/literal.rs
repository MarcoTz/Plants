use super::HtmlElement;
use crate::render::Render;

#[derive(Debug, PartialEq, Eq)]
pub struct Literal {
    content: String,
}

impl Render for Literal {
    fn render(&self) -> String {
        self.content.clone()
    }
}
impl From<Literal> for HtmlElement {
    fn from(lit: Literal) -> HtmlElement {
        HtmlElement::Literal(lit)
    }
}
impl From<String> for Literal {
    fn from(s: String) -> Literal {
        Literal { content: s }
    }
}

#[cfg(test)]
mod literal_tests {

    use super::{HtmlElement, Literal, Render};

    fn example_lit() -> Literal {
        Literal {
            content: "a literal".to_owned(),
        }
    }

    #[test]
    fn render_literal() {
        let result = example_lit().render();
        let expected = "a literal";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Literal as Into<HtmlElement>>::into(example_lit()).render();
        let expected = example_lit().render();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_from() {
        let result = <String as Into<Literal>>::into("a literal".to_owned()).render();
        let expected = example_lit().render();
        assert_eq!(result, expected)
    }
}
