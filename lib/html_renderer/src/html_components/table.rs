use super::component::{HtmlComponent, Render};
use std::rc::Rc;

pub struct Td {
    pub contents: Rc<HtmlComponent>,
}
pub struct Tr {
    pub class: Option<String>,
    pub id: Option<String>,
    pub cols: Vec<Td>,
}
pub struct Table {
    pub rows: Vec<Tr>,
}

impl Table {
    pub fn render(&self) -> String {
        let mut tr_str = "".to_owned();
        for tr in self.rows.iter() {
            tr_str.push_str(&tr.render());
        }
        format!("<table>{tr_str}</table>")
    }
}

impl Render for Tr {
    fn render(&self) -> String {
        let class_str = match self.class.clone() {
            None => "".to_owned(),
            Some(cl) => format!("class=\"{cl}\""),
        };
        let id_str = match self.id.clone() {
            None => "".to_owned(),
            Some(id) => format!("id=\"{id}\""),
        };
        let mut td_str = "".to_owned();
        for td in self.cols.iter() {
            td_str.push_str(&td.render());
        }
        format!("<tr {class_str} {id_str} >{td_str}</tr>")
    }
}

impl Render for Td {
    fn render(&self) -> String {
        let content_str = self.contents.render();
        format!("<td>{content_str}</td>")
    }
}

impl From<Table> for HtmlComponent {
    fn from(tb: Table) -> HtmlComponent {
        HtmlComponent::Table(tb)
    }
}
