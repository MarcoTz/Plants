use super::{
    elements::{body::Body, head::Head},
    render::Render,
};

pub struct HtmlDocument {
    pub head: Head,
    pub body: Body,
}

impl Render for HtmlDocument {
    fn render(&self) -> String {
        let head_str = self.head.render();
        let body_str = self.body.render();
        format!("<!doctype html>{head_str}{body_str}")
    }
}
