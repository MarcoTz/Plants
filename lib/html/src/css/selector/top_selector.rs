use super::Selector;
use crate::render::Render;

#[derive(Clone, PartialEq, Eq, Debug)]
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

#[cfg(test)]
mod top_selector_tests {

    use super::{Render, Selector, TopSelector};

    #[test]
    fn render_class() {
        let result = TopSelector::Class("class".to_owned()).render();
        let expected = ".class";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_id() {
        let result = TopSelector::Id("id".to_owned()).render();
        let expected = "#id";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_tag() {
        let result = TopSelector::Tag("tag".to_owned()).render();
        let expected = "tag";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_pseudo() {
        let result = TopSelector::Pseudo("root".to_owned()).render();
        let expected = ":root";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_all() {
        let result = TopSelector::All.render();
        let expected = "*";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_multiple() {
        let result = TopSelector::Multiple(vec![
            TopSelector::Id("id".to_owned()),
            TopSelector::Class("class".to_owned()),
        ])
        .render();
        let expected = "#id, .class";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <TopSelector as Into<Selector>>::into(TopSelector::All).render();
        let expected = TopSelector::All.render();
        assert_eq!(result, expected)
    }
}
