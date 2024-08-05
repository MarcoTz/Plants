use super::{
    body::Body, canvas::Canvas, div::Div, figure::Figure, head::Head, headline::Headline, img::Img,
    input::Input, link::Link, literal::Literal, select::Select, table::Table,
};

pub enum HtmlElement {
    Head(Head),
    Body(Body),
    Div(Div),
    Link(Link),
    Literal(Literal),
    Table(Table),
    Headline(Headline),
    Br,
    Figure(Figure),
    Img(Img),
    Canvas(Canvas),
    Input(Input),
    Select(Select),
    ComponentList(Vec<HtmlElement>),
}

pub struct Html {
    pub head: Head,
    pub body: Body,
}

pub trait Render {
    fn render(&self) -> String;
}

impl Render for HtmlElement {
    fn render(&self) -> String {
        match self {
            HtmlElement::Head(hd) => hd.render(),
            HtmlElement::Body(bd) => bd.render(),
            HtmlElement::Div(dv) => dv.render(),
            HtmlElement::Link(lnk) => lnk.render(),
            HtmlElement::Literal(lit) => lit.render(),
            HtmlElement::Table(tb) => tb.render(),
            HtmlElement::Headline(hd) => hd.render(),
            HtmlElement::Br => "<br/>".to_owned(),
            HtmlElement::Figure(fig) => fig.render(),
            HtmlElement::Img(img) => img.render(),
            HtmlElement::Canvas(canvas) => canvas.render(),
            HtmlElement::Input(input) => input.render(),
            HtmlElement::Select(select) => select.render(),
            HtmlElement::ComponentList(ls) => {
                let mut out_str = "".to_owned();
                for comp in ls.iter() {
                    out_str.push_str(&comp.render());
                }
                out_str
            }
        }
    }
}

impl<T: Render> Render for Vec<T> {
    fn render(&self) -> String {
        let mut out_str = "".to_owned();
        for elem in self.iter() {
            out_str.push_str(&elem.render());
        }
        out_str
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

impl Render for Html {
    fn render(&self) -> String {
        let head_str = self.head.render();
        let body_str = self.body.render();
        format!("<!doctype html>{head_str}{body_str}")
    }
}
