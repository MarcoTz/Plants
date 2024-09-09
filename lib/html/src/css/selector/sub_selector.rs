use super::Selector;
use crate::render::Render;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SubSelector {
    Visited,
    NthChild(ChildSelector),
    ChildCombinator(Rc<Selector>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ChildSelector {
    Odd,
    Even,
    AnPlusB(i32, i32),
}

impl Render for SubSelector {
    fn render(&self) -> String {
        match self {
            SubSelector::Visited => ":visited".to_owned(),
            SubSelector::NthChild(child_select) => {
                let child_select_str = child_select.render();
                format!(":nth-child({child_select_str})")
            }
            SubSelector::ChildCombinator(top_select) => {
                let top_str = top_select.render();
                format!(">{top_str}")
            }
        }
    }
}

impl Render for ChildSelector {
    fn render(&self) -> String {
        match self {
            ChildSelector::Odd => "odd".to_string(),
            ChildSelector::Even => "even".to_string(),
            ChildSelector::AnPlusB(a, b) => format!("{a}n+{b}"),
        }
    }
}

impl From<ChildSelector> for SubSelector {
    fn from(child_select: ChildSelector) -> SubSelector {
        SubSelector::NthChild(child_select)
    }
}

#[cfg(test)]
mod subselector_tests {
    use super::{ChildSelector, Render, SubSelector};
    use crate::css::selector::TopSelector;
    use std::rc::Rc;

    #[test]
    fn render_visited() {
        let result = SubSelector::Visited.render();
        let expected = ":visited";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_child_even() {
        let result = SubSelector::NthChild(ChildSelector::Even).render();
        let expected = ":nth-child(even)";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_child_odd() {
        let result = SubSelector::NthChild(ChildSelector::Odd).render();
        let expected = ":nth-child(odd)";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_child_linear() {
        let result = SubSelector::NthChild(ChildSelector::AnPlusB(3, 4)).render();
        let expected = ":nth-child(3n+4)";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_child_combinator() {
        let result =
            SubSelector::ChildCombinator(Rc::new(TopSelector::Tag("img".to_owned()).into()))
                .render();
        let expected = ">img";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_even() {
        let result = ChildSelector::Even.render();
        let expected = "even";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_odd() {
        let result = ChildSelector::Odd.render();
        let expected = "odd";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_linear() {
        let result = ChildSelector::AnPlusB(1, 2).render();
        let expected = "1n+2";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <ChildSelector as Into<SubSelector>>::into(ChildSelector::Even).render();
        let expected = ":nth-child(even)";
        assert_eq!(result, expected)
    }
}
