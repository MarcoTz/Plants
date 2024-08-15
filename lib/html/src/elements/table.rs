use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

pub struct Td {
    pub content: Rc<HtmlElement>,
}
pub struct Tr {
    pub attributes: Vec<Attribute>,
    pub cols: Vec<Td>,
}
pub struct Table {
    pub attributes: Vec<Attribute>,
    pub rows: Vec<HtmlElement>,
}

impl Render for Table {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace("\n", " ");
        let mut tr_str = "".to_owned();
        for tr in self.rows.iter() {
            tr_str.push_str(&tr.render());
        }
        tr_str = tr_str.replace("\n", "\n\t");
        format!("<table {attr_str}>\n\t{tr_str}\n</table>")
    }
}

impl Render for Tr {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace("\n", " ");
        let mut td_str = "".to_owned();
        for td in self.cols.iter() {
            td_str.push_str(&td.render());
        }
        td_str = td_str.replace("\n", "\n\t");
        format!("<tr {attr_str}>\n\t{td_str}\n</tr>")
    }
}

impl Render for Td {
    fn render(&self) -> String {
        let content_str = self.content.render().replace("\n", "\n\t");
        format!("<td>\n\t{content_str}\n</td>")
    }
}

impl From<Table> for HtmlElement {
    fn from(tb: Table) -> HtmlElement {
        HtmlElement::Table(tb)
    }
}

impl From<Tr> for HtmlElement {
    fn from(tr: Tr) -> HtmlElement {
        HtmlElement::Tr(tr)
    }
}

impl From<Td> for HtmlElement {
    fn from(td: Td) -> HtmlElement {
        HtmlElement::Td(td)
    }
}
