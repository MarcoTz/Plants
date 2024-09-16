mod a;
mod body;
mod canvas;
mod div;
mod figure;
mod head;
mod headline;
mod img;
mod input;
mod link;
mod literal;
mod script;
mod select;
mod style;
mod table;

pub use a::A;
pub use body::Body;
pub use canvas::Canvas;
pub use div::Div;
pub use figure::Figure;
pub use head::Head;
pub use headline::{HeaderSize, Headline};
pub use img::Img;
pub use input::Input;
pub use link::Link;
pub use literal::Literal;
pub use script::Script;
pub use select::{Select, SelectOption};
pub use style::Style;
pub use table::{Table, Td, Tr};

use crate::render::Render;

#[derive(Debug, PartialEq)]
pub enum HtmlElement {
    Head(Head),
    Body(Body),
    Div(Div),
    Link(Link),
    A(A),
    Literal(Literal),
    Table(Table),
    Tr(Tr),
    Td(Td),
    Headline(Headline),
    Br,
    Figure(Figure),
    Img(Img),
    Canvas(Canvas),
    Input(Input),
    Select(Select),
    Script(Script),
    ComponentList(Vec<HtmlElement>),
    Style(Style),
}

impl Render for HtmlElement {
    fn render(&self) -> String {
        match self {
            HtmlElement::Head(hd) => hd.render(),
            HtmlElement::Body(bd) => bd.render(),
            HtmlElement::Div(dv) => dv.render(),
            HtmlElement::A(a) => a.render(),
            HtmlElement::Link(lnk) => lnk.render(),
            HtmlElement::Literal(lit) => lit.render(),
            HtmlElement::Table(tb) => tb.render(),
            HtmlElement::Tr(tr) => tr.render(),
            HtmlElement::Td(td) => td.render(),
            HtmlElement::Headline(hd) => hd.render(),
            HtmlElement::Br => "<br/>".to_owned(),
            HtmlElement::Figure(fig) => fig.render(),
            HtmlElement::Img(img) => img.render(),
            HtmlElement::Canvas(canvas) => canvas.render(),
            HtmlElement::Input(input) => input.render(),
            HtmlElement::Select(select) => select.render(),
            HtmlElement::Script(script) => script.render(),
            HtmlElement::Style(style) => style.render(),
            HtmlElement::ComponentList(ls) => ls.render(),
        }
    }
}

impl From<String> for HtmlElement {
    fn from(s: String) -> HtmlElement {
        HtmlElement::Literal(s.into())
    }
}

impl From<Vec<HtmlElement>> for HtmlElement {
    fn from(ls: Vec<HtmlElement>) -> HtmlElement {
        HtmlElement::ComponentList(ls)
    }
}

#[cfg(test)]
mod elements_tests {
    use super::{
        Body, Canvas, Div, Figure, Head, HeaderSize, Headline, HtmlElement, Img, Input, Link,
        Literal, Render, Script, Select, Style, Table, Td, Tr, A,
    };
    use crate::css::CssDocument;
    use std::rc::Rc;

    #[test]
    fn render_head() {
        let result = HtmlElement::Head(Head {
            title: "a head".to_owned(),
            content: Rc::new(vec![].into()),
        })
        .render();
        let expected = "<head>\n\t<title>\n\t\ta head\n\t</title>\n\t\n</head>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_body() {
        let result = HtmlElement::Body(Body {
            attributes: vec![],
            content: Rc::new("body".to_owned().into()),
        })
        .render();
        let expected = "<body >\n\tbody\n</body>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_div() {
        let result = HtmlElement::Div(Div {
            attributes: vec![],
            content: Rc::new("div".to_owned().into()),
        })
        .render();
        let expected = "<div >\n\tdiv\n</div>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_a() {
        let result = HtmlElement::A(A {
            attributes: vec![],
            content: Rc::new("a".to_owned().into()),
        })
        .render();
        let expected = "<a >a</a>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_link() {
        let result = HtmlElement::Link(Link { attributes: vec![] }).render();
        let expected = "<link />";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_literal() {
        let result =
            HtmlElement::Literal(<String as Into<Literal>>::into("lit".to_owned())).render();
        let expected = "lit";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_table() {
        let result = HtmlElement::Table(Table {
            attributes: vec![],
            rows: vec![],
        })
        .render();
        let expected = "<table >\n\t\n</table>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_tr() {
        let result = HtmlElement::Tr(Tr {
            attributes: vec![],
            cols: vec![],
        })
        .render();
        let expected = "<tr >\n\t\n</tr>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_td() {
        let result = HtmlElement::Td(Td {
            content: Rc::new("td".to_owned().into()),
        })
        .render();
        let expected = "<td>\n\ttd\n</td>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_headline() {
        let result = HtmlElement::Headline(Headline {
            size: HeaderSize::H1,
            attributes: vec![],
            content: Rc::new("header".to_owned().into()),
        })
        .render();
        let expected = "<h1 >\n\theader\n</h1>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_br() {
        let result = HtmlElement::Br.render();
        let expected = "<br/>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_figure() {
        let result = HtmlElement::Figure(Figure {
            attributes: vec![],
            content: Rc::new("figure".to_owned().into()),
            caption: Rc::new("caption".to_owned().into()),
        })
        .render();
        let expected =
            "<figure >\n\tfigure\n\t<figcaption>\n\t\tcaption\n\t</figcaption>\n</figure>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_img() {
        let result = HtmlElement::Img(Img { attributes: vec![] }).render();
        let expected = "<img  />";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_canvas() {
        let result = HtmlElement::Canvas(Canvas { attributes: vec![] }).render();
        let expected = "<canvas ></canvas>";
        assert_eq!(result, expected)
    }

    #[test]
    fn text_input() {
        let result = HtmlElement::Input(Input { attributes: vec![] }).render();
        let expected = "<input />";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_select() {
        let result = HtmlElement::Select(Select {
            attributes: vec![],
            options: vec![],
        })
        .render();
        let expected = "<select >\n\t\n</select>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_script() {
        let result = HtmlElement::Script(Script {
            attributes: vec![],
            content: "".to_owned(),
        })
        .render();
        let expected = "<script >\n\t\n</script>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_style() {
        let result = HtmlElement::Style(Style {
            style: CssDocument { decls: vec![] },
        })
        .render();
        let expected = "<style>\n\t\n</style>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_component_list() {
        let result = HtmlElement::ComponentList(vec![]).render();
        let expected = "";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_str() {
        let result = <String as Into<HtmlElement>>::into("str".to_owned()).render();
        let expected = "str";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_vec() {
        let result = <Vec<HtmlElement> as Into<HtmlElement>>::into(vec![]).render();
        let expected = "";
        assert_eq!(result, expected)
    }
}
