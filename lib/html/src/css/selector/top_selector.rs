use super::Selector;
use crate::render::Render;

#[derive(Clone)]
pub enum TopSelector {
    Class(String),
    Id(String),
    Tag(String),
    Pseudo(String),
    All,
    Multiple(Vec<TopSelector>),
}

impl Render for TopSelector {
    fn render(&self) -> String {
        match self {
            TopSelector::Class(class) => format!(".{class}"),
            TopSelector::Id(id) => format!("#{id}"),
            TopSelector::Tag(tag) => tag.to_string(),
            TopSelector::Pseudo(pseudo) => format!(":{pseudo}"),
            TopSelector::All => "*".to_owned(),
            TopSelector::Multiple(selectors) => {
                let sel_strs: Vec<String> = selectors.iter().map(|s| s.render()).collect();
                sel_strs.join(", ")
            }
        }
    }
}

impl From<TopSelector> for Selector {
    fn from(top: TopSelector) -> Selector {
        Selector { top, sub: None }
    }
}
