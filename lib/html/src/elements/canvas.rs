use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};

#[derive(Debug, PartialEq, Eq)]
pub struct Canvas {
    pub attributes: Vec<Attribute>,
}

impl Render for Canvas {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace('\n', " ");
        format!("<canvas {attr_str}></canvas>")
    }
}

impl From<Canvas> for HtmlElement {
    fn from(canvas: Canvas) -> HtmlElement {
        HtmlElement::Canvas(canvas)
    }
}

#[cfg(test)]
mod canvas_tests {

    use super::{Attribute, Canvas, HtmlElement, Render};

    fn example_canvas() -> Canvas {
        Canvas {
            attributes: vec![Attribute::Id("canvas".to_owned())],
        }
    }

    #[test]
    fn render_canvas() {
        let result = example_canvas().render();
        let expected = "<canvas id=\"canvas\"></canvas>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Canvas as Into<HtmlElement>>::into(example_canvas()).render();
        let expected = example_canvas().render();
        assert_eq!(result, expected)
    }
}
