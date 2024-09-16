use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum HeaderSize {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug, PartialEq)]
pub struct Headline {
    pub size: HeaderSize,
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

impl From<Headline> for HtmlElement {
    fn from(hd: Headline) -> HtmlElement {
        HtmlElement::Headline(hd)
    }
}
impl Render for Headline {
    fn render(&self) -> String {
        let size_tag = self.size.render();
        let content_str = self.content.render().replace('\n', "\n\t");
        let attr_str = self.attributes.render().replace('\n', " ");
        format!("<{size_tag} {attr_str}>\n\t{content_str}\n</{size_tag}>")
    }
}

impl Render for HeaderSize {
    fn render(&self) -> String {
        match self {
            HeaderSize::H1 => "h1".to_owned(),
            HeaderSize::H2 => "h2".to_owned(),
            HeaderSize::H3 => "h3".to_owned(),
            HeaderSize::H4 => "h4".to_owned(),
            HeaderSize::H5 => "h5".to_owned(),
            HeaderSize::H6 => "h6".to_owned(),
        }
    }
}

#[cfg(test)]
mod headline_tests {
    use super::{Attribute, HeaderSize, Headline, HtmlElement, Render};
    use std::rc::Rc;

    fn example_headline() -> Headline {
        Headline {
            size: HeaderSize::H1,
            attributes: vec![Attribute::Id("header".to_owned())],
            content: Rc::new("A big headline".to_owned().into()),
        }
    }

    #[test]
    fn render_header() {
        let result = example_headline().render();
        let expected = "<h1 id=\"header\">\n\tA big headline\n</h1>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Headline as Into<HtmlElement>>::into(example_headline()).render();
        let expected = example_headline().render();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_h1() {
        let result = HeaderSize::H1.render();
        let expected = "h1";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_h2() {
        let result = HeaderSize::H2.render();
        let expected = "h2";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_h3() {
        let result = HeaderSize::H3.render();
        let expected = "h3";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_h4() {
        let result = HeaderSize::H4.render();
        let expected = "h4";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_h5() {
        let result = HeaderSize::H5.render();
        let expected = "h5";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_h6() {
        let result = HeaderSize::H6.render();
        let expected = "h6";
        assert_eq!(result, expected)
    }
}
