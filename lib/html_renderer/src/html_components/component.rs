use super::{
    body::Body, div::Div, figure::Figure, head::Head, headline::Headline, img::Img, link::Link,
    literal::Literal, table::Table,
};

pub enum HtmlComponent {
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
    ComponentList(Vec<HtmlComponent>),
}

pub struct Html {
    pub head: Head,
    pub body: Body,
}

pub trait Render {
    fn render(&self) -> String;
}

impl Render for HtmlComponent {
    fn render(&self) -> String {
        match self {
            HtmlComponent::Head(hd) => hd.render(),
            HtmlComponent::Body(bd) => bd.render(),
            HtmlComponent::Div(dv) => dv.render(),
            HtmlComponent::Link(lnk) => lnk.render(),
            HtmlComponent::Literal(lit) => lit.render(),
            HtmlComponent::Table(tb) => tb.render(),
            HtmlComponent::Headline(hd) => hd.render(),
            HtmlComponent::Br => "<br/>".to_owned(),
            HtmlComponent::Figure(fig) => fig.render(),
            HtmlComponent::Img(img) => img.render(),
            HtmlComponent::ComponentList(ls) => {
                let mut out_str = "".to_owned();
                for comp in ls.iter() {
                    out_str.push_str(&comp.render());
                }
                out_str
            }
        }
    }
}

impl From<String> for HtmlComponent {
    fn from(s: String) -> HtmlComponent {
        HtmlComponent::Literal(s.into())
    }
}

impl From<Vec<HtmlComponent>> for HtmlComponent {
    fn from(ls: Vec<HtmlComponent>) -> HtmlComponent {
        HtmlComponent::ComponentList(ls)
    }
}

impl Render for Html {
    fn render(&self) -> String {
        let head_str = self.head.render();
        let body_str = self.body.render();
        format!("<!doctype html>{head_str}{body_str}")
    }
}
