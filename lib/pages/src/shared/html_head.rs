use crate::{css::PageCss, page::PageComponent};
use html::{
    attribute::Attribute,
    elements::{Head, HtmlElement, Script},
};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HtmlHead {
    pub title: String,
    pub styles: PageCss,
    pub scripts: Vec<String>,
    pub date_format: String,
}

impl From<&HtmlHead> for Head {
    fn from(hd: &HtmlHead) -> Head {
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
            content: Rc::new(vec![hd.styles.render(&hd.date_format), scripts.into()].into()),
        }
    }
}
impl PageComponent for HtmlHead {
    fn render(&self, _: &str) -> HtmlElement {
        Head::from(self).into()
    }
}
