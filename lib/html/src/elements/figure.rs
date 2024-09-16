use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Figure {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
    pub caption: Rc<HtmlElement>,
}

impl Render for Figure {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace('\n', " ");
        let contents_str = self.content.render().replace('\n', "\n\t");
        let caption_str = self.caption.render().replace('\n', "\n\t\t");
        format!("<figure {attr_str}>\n\t{contents_str}\n\t<figcaption>\n\t\t{caption_str}\n\t</figcaption>\n</figure>")
    }
}

impl From<Figure> for HtmlElement {
    fn from(fig: Figure) -> HtmlElement {
        HtmlElement::Figure(fig)
    }
}

#[cfg(test)]
mod figure_tests {

    use super::{Attribute, Figure, HtmlElement, Render};
    use std::rc::Rc;

    fn example_figure() -> Figure {
        Figure {
            attributes: vec![Attribute::Id("fig".to_owned())],
            content: Rc::new("a figure".to_owned().into()),
            caption: Rc::new("a caption".to_owned().into()),
        }
    }

    #[test]
    fn render_figure() {
        let result = example_figure().render();
        let expected =
            "<figure id=\"fig\">\n\ta figure\n\t<figcaption>\n\t\ta caption\n\t</figcaption>\n</figure>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Figure as Into<HtmlElement>>::into(example_figure()).render();
        let expected = example_figure().render();
        assert_eq!(result, expected)
    }
}
