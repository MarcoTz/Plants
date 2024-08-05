use super::{
    super::html::{body::Body, head::Head, html_element::Html},
    components::{footer::Footer, graveyard_table::GraveyardTable, header::Header},
    page::{Page, PageComponent},
};
use std::rc::Rc;
pub struct Graveyard {
    header: Header,
    footer: Footer,
    graveyard_table: GraveyardTable,
}

impl Page for Graveyard {
    fn render(&self) -> Html {
        let head = Head {
            title: "Graveyard".to_owned(),
        };
        let body_contents = vec![
            self.header.render(),
            self.graveyard_table.render(),
            self.footer.render(),
        ];
        let body = Body {
            content: Rc::new(body_contents.into()),
        };
        Html { head, body }
    }
}
