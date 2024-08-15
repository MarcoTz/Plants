use super::{
    elements::{Body, Head},
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
        format!("<!doctype html>\n<html>\n\t{head_str}\n\t{body_str}</html>")
    }
}
