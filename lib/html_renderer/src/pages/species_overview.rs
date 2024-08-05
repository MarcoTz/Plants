use super::{
    super::html::{body::Body, head::Head, html_document::HtmlDocument},
    components::{footer::Footer, header::Header, species_list::SpeciesList},
    page::{Page, PageComponent},
};
use std::rc::Rc;

pub struct SpeciesOverview {
    header: Header,
    footer: Footer,
    species_list: SpeciesList,
}

impl Page for SpeciesOverview {
    fn render(&self) -> HtmlDocument {
        let head = Head {
            title: "All Plant Species".to_owned(),
        };
        let body_contents = vec![
            self.header.render(),
            self.species_list.render(),
            self.footer.render(),
        ];
        let body = Body {
            content: Rc::new(body_contents.into()),
        };
        HtmlDocument { head, body }
    }
}
