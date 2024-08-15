pub mod activities_table;
use super::{
    css::PageCss,
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use activities_table::ActivitiesTable;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement},
};
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

    fn get_head(&self, date_format: &str) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: "Activities".to_owned(),
            styles: PageCss::Activities,
            scripts,
            date_format: date_format.to_owned(),
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
