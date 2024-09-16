mod sub_selector;
mod top_selector;

pub use sub_selector::{ChildSelector, SubSelector};
pub use top_selector::TopSelector;

use crate::render::Render;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selector {
    pub top: TopSelector,
    pub sub: Option<SubSelector>,
}

impl Render for Selector {
    fn render(&self) -> String {
        let top_str = self.top.render();
        let sub_str = self
            .sub
            .as_ref()
            .map(|sb| sb.render())
            .unwrap_or("".to_owned());
        format!("{top_str}{sub_str}")
    }
}

#[cfg(test)]
mod selector_tests {

    use super::{Render, Selector, SubSelector, TopSelector};

    #[test]
    fn render_top_only() {
        let result = Selector {
            top: TopSelector::All,
            sub: None,
        }
        .render();
        let expected = "*";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_top_sub() {
        let result = Selector {
            top: TopSelector::Id("ident".to_owned()),
            sub: Some(SubSelector::Visited),
        }
        .render();
        let expected = "#ident:visited";
        assert_eq!(result, expected)
    }
}
