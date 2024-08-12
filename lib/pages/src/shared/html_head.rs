use crate::components::page_component::PageComponent;
use html::{attribute::Attribute, head::Head, html_element::HtmlElement, link::Link};
use std::rc::Rc;

#[derive(Clone)]
pub struct HtmlHead {
    pub title: String,
    pub styles: Vec<String>,
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
        Head {
            title: hd.title.clone(),
            content: Rc::new(styles.into()),
        }
    }
}
impl PageComponent for HtmlHead {
    fn render(&self, _: &str) -> HtmlElement {
        Head::from(self).into()
    }
}
