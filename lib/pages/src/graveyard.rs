use super::{
    components::{graveyard_table::GraveyardTable, page_component::PageComponent},
    page::Page,
    shared::{footer::Footer, header::Header, html_head::HtmlHead},
};
use html::{body::Body, head::Head, html_document::HtmlDocument};
use plants::graveyard::GraveyardPlant;
use std::rc::Rc;

pub struct Graveyard {
    pub header: Header,
    pub footer: Footer,
    pub graveyard_table: GraveyardTable,
}

impl Page for Graveyard {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let body_contents = vec![
            self.header.render(date_format),
            self.graveyard_table.render(date_format),
            self.footer.render(date_format),
        ];
        let body = Body {
            content: Rc::new(body_contents.into()),
        };
        HtmlDocument {
            head: Head::from(&self.get_head()),
            body,
        }
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "css/main.css".to_owned(),
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
        ];
        HtmlHead {
            title: "Graveyard".to_owned(),
            styles,
        }
    }
}

impl From<(&[GraveyardPlant], i32)> for Graveyard {
    fn from((graveyard, num_plants): (&[GraveyardPlant], i32)) -> Graveyard {
        Graveyard {
            header: Header::from(false),
            footer: Footer::from(num_plants),
            graveyard_table: GraveyardTable::from(graveyard),
        }
    }
}
