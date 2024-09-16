use crate::elements::HtmlElement;
use crate::{css::CssDocument, render::Render};

#[derive(Debug, PartialEq)]
pub struct Style {
    pub style: CssDocument,
}

impl Render for Style {
    fn render(&self) -> String {
        let blocks_str = self.style.render().replace('\n', "\n\t");
        format!("<style>\n\t{blocks_str}\n</style>")
    }
}

impl From<Style> for HtmlElement {
    fn from(style: Style) -> HtmlElement {
        HtmlElement::Style(style)
    }
}

#[cfg(test)]
mod styles_tests {

    use super::{HtmlElement, Render, Style};
    use crate::css::{
        block::CssBlock,
        property::{Property, Size},
        selector::TopSelector,
        value::{Keyword, Unit},
        CssDocument,
    };

    fn example_style() -> Style {
        Style {
            style: CssDocument {
                decls: vec![
                    CssBlock {
                        selector: TopSelector::All.into(),
                        decls: vec![
                            (Property::Display, Keyword::Non.into()).into(),
                            (Size::Height.into(), (0.0, Unit::Percent.into()).into()).into(),
                        ],
                    },
                    CssBlock {
                        selector: TopSelector::Tag("div".to_owned()).into(),
                        decls: vec![(Property::Display, Keyword::Flex.into()).into()],
                    },
                ],
            },
        }
    }

    #[test]
    fn render_style() {
        let result = example_style().render();
        let expected =
            "<style>\n\t* {\n\t\tdisplay:none;\n\t\theight:0%;\n\t}\n\tdiv {\n\t\tdisplay:flex;\n\t}\n</style>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Style as Into<HtmlElement>>::into(example_style()).render();
        let expected = example_style().render();
        assert_eq!(result, expected)
    }
}
