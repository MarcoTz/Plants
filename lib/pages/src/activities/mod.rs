pub mod activities_table;
use super::{
    css::DefinedDocument,
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use activities_table::ActivitiesTable;
use html::{
    attribute::Attribute,
    elements::{div::Div, HtmlElement},
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

    fn get_head(&self) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: "Activities".to_owned(),
            styles_extern: vec![],
            styles: vec![
                DefinedDocument::Main,
                DefinedDocument::Header,
                DefinedDocument::Footer,
            ],
            scripts,
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
