use super::CssDocument;
use super::{declaration::Declaration, selector::Selector};
use crate::render::Render;

#[derive(Clone, Debug, PartialEq)]
pub struct CssBlock {
    pub selector: Selector,
    pub decls: Vec<Declaration>,
}

impl Render for CssBlock {
    fn render(&self) -> String {
        let selector_str = self.selector.render();
        let decls_str = self.decls.render().replace('\n', "\n\t");

        format!("{selector_str} {{\n\t{decls_str}\n}}")
    }
}

impl From<Vec<CssBlock>> for CssDocument {
    fn from(blocks: Vec<CssBlock>) -> CssDocument {
        CssDocument { decls: blocks }
    }
}

#[cfg(test)]
mod css_block_tests {
    use super::{CssBlock, CssDocument, Render};
    use crate::css::{
        declaration::Declaration, property::Property, selector::TopSelector, value::Keyword,
    };

    fn example_block() -> CssBlock {
        CssBlock {
            selector: TopSelector::Id("container".to_owned()).into(),
            decls: vec![Declaration {
                property: Property::Display,
                value: Keyword::Non.into(),
                important: false,
            }],
        }
    }

    #[test]
    fn render_block() {
        let result = example_block().render();
        let expected = "#container {\n\tdisplay:none;\n}";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Vec<CssBlock> as Into<CssDocument>>::into(vec![example_block()]).render();
        let expected = example_block().render();
        assert_eq!(result, expected)
    }
}
