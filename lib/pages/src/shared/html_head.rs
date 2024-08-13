use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{head::Head, link::Link, script::Script, HtmlElement},
};
use std::rc::Rc;

#[derive(Clone)]
pub struct HtmlHead {
    pub title: String,
    pub styles: Vec<String>,
    pub scripts: Vec<String>,
}

impl From<&HtmlHead> for Head {
    fn from(hd: &HtmlHead) -> Head {
        let mut styles = vec![];
        for style in hd.styles.iter() {
            styles.push(
                Link {
                    attributes: vec![
                        Attribute::Href(style.clone()),
                        Attribute::Rel("stylesheet".to_owned()),
                    ],
                }
                .into(),
            );
        }
        let mut scripts = vec![];
        for script in hd.scripts.iter() {
            scripts.push(
                Script {
                    attributes: vec![Attribute::Src(script.clone())],
                    content: "".to_owned(),
                }
                .into(),
            )
        }
        Head {
            title: hd.title.clone(),
            content: Rc::new(vec![styles.into(), scripts.into()].into()),
        }
    }
}
impl PageComponent for HtmlHead {
    fn render(&self, _: &str) -> HtmlElement {
        Head::from(self).into()
    }
}
