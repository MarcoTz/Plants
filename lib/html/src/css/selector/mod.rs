mod sub_selector;
mod top_selector;

pub use sub_selector::{ChildSelector, SubSelector};
pub use top_selector::TopSelector;

use crate::render::Render;
#[derive(Clone)]
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
