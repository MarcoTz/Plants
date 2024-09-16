use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    pub attributes: Vec<Attribute>,
}

impl Render for Input {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace('\n', " ");
        format!("<input {attr_str}/>")
    }
}
impl From<Input> for HtmlElement {
    fn from(input: Input) -> HtmlElement {
        HtmlElement::Input(input)
    }
}

#[cfg(test)]
mod input_test {

    use super::{Attribute, HtmlElement, Input, Render};

    fn example_input() -> Input {
        Input {
            attributes: vec![Attribute::Id("name".to_owned())],
        }
    }

    #[test]
    fn render_input() {
        let result = example_input().render();
        let expected = "<input id=\"name\"/>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Input as Into<HtmlElement>>::into(example_input()).render();
        let expected = example_input().render();
        assert_eq!(result, expected)
    }
}
