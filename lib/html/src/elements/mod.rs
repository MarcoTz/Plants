pub mod a;
pub mod body;
pub mod canvas;
pub mod div;
pub mod figure;
pub mod head;
pub mod headline;
pub mod img;
pub mod input;
pub mod link;
pub mod literal;
pub mod script;
pub mod select;
pub mod style;
pub mod table;

use a::A;
use body::Body;
use canvas::Canvas;
use div::Div;
use figure::Figure;
use head::Head;
use headline::Headline;
use img::Img;
use input::Input;
use link::Link;
use literal::Literal;
use script::Script;
use select::Select;
use style::Style;
use table::{Table, Td, Tr};

use crate::render::Render;

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
