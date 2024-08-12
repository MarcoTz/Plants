pub mod activities_table;
use super::{components::page_component::PageComponent, page::Page, shared::html_head::HtmlHead};
use activities_table::ActivitiesTable;
use html::{attribute::Attribute, div::Div, html_element::HtmlElement};
use plants::plant::Plant;
use std::rc::Rc;

pub struct Activities {
    pub activity_table: ActivitiesTable,
}

impl Page for Activities {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Style("width:95%;margin:auto;".to_owned())],
            content: Rc::new(self.activity_table.render(date_format)),
        }
        .into()
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
            activity_table: ActivitiesTable::from(plants),
        }
    }
}
