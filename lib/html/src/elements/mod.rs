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
