use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};

#[derive(Debug, PartialEq, Eq)]
pub struct Link {
    pub attributes: Vec<Attribute>,
}

impl Render for Link {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace('\n', " ");
        format!("<link {attr_str}/>")
    }
}

impl From<Link> for HtmlElement {
    fn from(lnk: Link) -> HtmlElement {
        HtmlElement::Link(lnk)
    }
}

#[cfg(test)]
mod link_tests {

    use super::{Attribute, HtmlElement, Link, Render};

    fn example_link() -> Link {
        Link {
            attributes: vec![Attribute::Id("link".to_owned())],
        }
    }

    #[test]
    fn render_link() {
        let result = example_link().render();
        let expected = "<link id=\"link\"/>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Link as Into<HtmlElement>>::into(example_link()).render();
        let expected = example_link().render();
        assert_eq!(result, expected)
    }
}
