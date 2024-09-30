use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Td {
    pub content: Rc<HtmlElement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tr {
    pub attributes: Vec<Attribute>,
    pub cols: Vec<Td>,
}

#[derive(Debug, PartialEq)]
pub struct Table {
    pub attributes: Vec<Attribute>,
    pub rows: Vec<HtmlElement>,
}

impl Render for Table {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace('\n', " ");
        let mut tr_str = "".to_owned();
        for tr in self.rows.iter() {
            tr_str.push_str(&tr.render());
        }
        tr_str = tr_str.replace('\n', "\n\t");
        format!("<table {attr_str}>\n\t{tr_str}\n</table>")
    }
}

impl Render for Tr {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace('\n', " ");
        let mut td_str = "".to_owned();
        for td in self.cols.iter() {
            td_str.push_str(&td.render());
        }
        td_str = td_str.replace('\n', "\n\t");
        format!("<tr {attr_str}>\n\t{td_str}\n</tr>")
    }
}

impl Render for Td {
    fn render(&self) -> String {
        let content_str = self.content.render().replace('\n', "\n\t");
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

#[cfg(test)]
mod table_tests {
    use super::{Attribute, HtmlElement, Render, Table, Td, Tr};
    use std::rc::Rc;

    fn example_td() -> Td {
        Td {
            content: Rc::new("td".to_owned().into()),
        }
    }

    fn example_tr() -> Tr {
        Tr {
            attributes: vec![Attribute::Id("row".to_owned())],
            cols: vec![example_td()],
        }
    }

    fn example_table() -> Table {
        Table {
            attributes: vec![Attribute::Id("table".to_owned())],
            rows: vec![example_tr().into()],
        }
    }

    #[test]
    fn render_table() {
        let result = example_table().render();
        let expected =
            "<table id=\"table\">\n\t<tr id=\"row\">\n\t\t<td>\n\t\t\ttd\n\t\t</td>\n\t</tr>\n</table>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_tr() {
        let result = example_tr().render();
        let expected = "<tr id=\"row\">\n\t<td>\n\t\ttd\n\t</td>\n</tr>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_td() {
        let result = example_td().render();
        let expected = "<td>\n\ttd\n</td>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_table_into() {
        let result = <Table as Into<HtmlElement>>::into(example_table()).render();
        let expected = example_table().render();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_tr_into() {
        let result = <Tr as Into<HtmlElement>>::into(example_tr()).render();
        let expected = example_tr().render();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_td_into() {
        let result = <Td as Into<HtmlElement>>::into(example_td()).render();
        let expected = example_td().render();
        assert_eq!(result, expected)
    }
}
