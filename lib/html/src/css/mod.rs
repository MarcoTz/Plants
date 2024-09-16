pub mod block;
pub mod declaration;
pub mod property;
pub mod selector;
pub mod value;

use crate::{
    elements::{HtmlElement, Style},
    render::Render,
};
use block::CssBlock;

#[derive(Clone, Debug, PartialEq)]
pub struct CssDocument {
    pub decls: Vec<CssBlock>,
}

impl Render for CssDocument {
    fn render(&self) -> String {
        self.decls.render()
    }
}

impl From<CssDocument> for HtmlElement {
    fn from(css: CssDocument) -> HtmlElement {
        HtmlElement::Style(Style { style: css })
    }
}

#[cfg(test)]
mod css_tests {
    use super::{CssDocument, HtmlElement, Render};
    use crate::css::{block::CssBlock, property::Property, selector::TopSelector, value::Keyword};

    fn example_document() -> CssDocument {
        CssDocument {
            decls: vec![
                CssBlock {
                    selector: TopSelector::Class("overflower".to_owned()).into(),
                    decls: vec![(Property::Overflow, Keyword::Auto.into()).into()],
                },
                CssBlock {
                    selector: TopSelector::Class("notoverflower".to_owned()).into(),
                    decls: vec![(Property::Overflow, Keyword::Non.into()).into()],
                },
            ],
        }
    }

    #[test]
    fn render_empty() {
        let result = CssDocument { decls: vec![] }.render();
        let expected = "";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_doc() {
        let result = example_document().render();
        let expected = ".overflower {\n\toverflow:auto;\n}\n.notoverflower {\n\toverflow:none;\n}";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <CssDocument as Into<HtmlElement>>::into(example_document()).render();
        let expected = "<style>\n\t".to_owned()
            + &example_document().render().replace("\n", "\n\t")
            + "\n</style>";
        assert_eq!(result, expected)
    }
}
