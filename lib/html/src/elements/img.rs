use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};

#[derive(Debug, PartialEq, Eq)]
pub struct Img {
    pub attributes: Vec<Attribute>,
}

impl Render for Img {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace('\n', " ");
        format!("<img {attr_str} />")
    }
}

impl From<Img> for HtmlElement {
    fn from(img: Img) -> HtmlElement {
        HtmlElement::Img(img)
    }
}

#[cfg(test)]
mod img_tests {

    use super::{Attribute, HtmlElement, Img, Render};

    fn example_img() -> Img {
        Img {
            attributes: vec![
                Attribute::Id("img".to_owned()),
                Attribute::Src("img.jpg".to_owned()),
            ],
        }
    }

    #[test]
    fn render_img() {
        let result = example_img().render();
        let expected = "<img id=\"img\" src=\"img.jpg\" />";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Img as Into<HtmlElement>>::into(example_img()).render();
        let expected = example_img().render();
        assert_eq!(result, expected)
    }
}
