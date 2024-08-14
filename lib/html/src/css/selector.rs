use crate::render::Render;
use std::rc::Rc;

#[derive(Clone)]
pub struct Selector {
    pub top: TopSelector,
    pub sub: Option<SubSelector>,
}

#[derive(Clone)]
pub enum TopSelector {
    Class(String),
    Id(String),
    Tag(String),
    Pseudo(String),
    All,
}

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

impl Render for TopSelector {
    fn render(&self) -> String {
        match self {
            TopSelector::Class(class) => format!(".{class}"),
            TopSelector::Id(id) => format!("#{id}"),
            TopSelector::Tag(tag) => tag.to_string(),
            TopSelector::Pseudo(pseudo) => format!(":{pseudo}"),
            TopSelector::All => "*".to_owned(),
        }
    }
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

impl From<TopSelector> for Selector {
    fn from(top: TopSelector) -> Selector {
        Selector { top, sub: None }
    }
}

impl From<ChildSelector> for SubSelector {
    fn from(child_select: ChildSelector) -> SubSelector {
        SubSelector::NthChild(child_select)
    }
}
