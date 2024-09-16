use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};

#[derive(Debug, PartialEq, Eq)]
pub struct Script {
    pub attributes: Vec<Attribute>,
    pub content: String,
}

impl Render for Script {
    fn render(&self) -> String {
        let content_str = self.content.clone().replace('\n', "\n\t");
        let attr_str = self.attributes.render().replace('\n', " ");
        format!("<script {attr_str}>\n\t{content_str}\n</script>")
    }
}

impl From<Script> for HtmlElement {
    fn from(script: Script) -> HtmlElement {
        HtmlElement::Script(script)
    }
}

#[cfg(test)]
mod script_tests {
    use super::{Attribute, HtmlElement, Render, Script};

    fn example_script() -> Script {
        Script {
            attributes: vec![Attribute::Type("javascript".to_owned())],
            content: "alert(\"hello\")".to_owned(),
        }
    }

    #[test]
    fn render_script() {
        let result = example_script().render();
        let expected = "<script type=\"javascript\">\n\talert(\"hello\")\n</script>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Script as Into<HtmlElement>>::into(example_script()).render();
        let expected = example_script().render();
        assert_eq!(result, expected)
    }
}
