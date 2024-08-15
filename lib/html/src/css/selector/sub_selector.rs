use super::Selector;
use crate::render::Render;
use std::rc::Rc;

#[derive(Clone)]
pub enum SubSelector {
    Visited,
    NthChild(ChildSelector),
    ChildCombinator(Rc<Selector>),
}

#[derive(Clone)]
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
