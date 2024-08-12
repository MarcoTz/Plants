use super::{
    components::{page_component::PageComponent, plant_activity_table::PlantActivityTable},
    page::Page,
    shared::{footer::Footer, header::Header, html_head::HtmlHead},
};

use html::{body::Body, head::Head, html_document::HtmlDocument};
use plants::plant::Plant;
use std::rc::Rc;

pub struct Activities {
    pub header: Header,
    pub footer: Footer,
    pub activity_table: PlantActivityTable,
}

impl Page for Activities {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let body_contents = vec![
            self.header.render(date_format),
            self.activity_table.render(date_format),
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
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
            "css/main.css".to_owned(),
        ];
        HtmlHead {
            title: "Activities".to_owned(),
            styles,
        }
    }
}

impl From<&[Plant]> for Activities {
    fn from(plants: &[Plant]) -> Activities {
        Activities {
            header: Header::from(false),
            activity_table: PlantActivityTable::from((plants, true, true)),
            footer: Footer::from(plants.len() as i32),
        }
    }
}
