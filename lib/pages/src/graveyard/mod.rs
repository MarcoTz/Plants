pub mod graveyard_table;

use super::{
    css::PageCss,
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use graveyard_table::GraveyardTable;
use html::elements::HtmlElement;
use plants::graveyard::GraveyardPlant;

pub struct Graveyard {
    pub graveyard_table: GraveyardTable,
}

impl Page for Graveyard {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        self.graveyard_table.render(date_format)
    }

    fn get_head(&self, date_format: &str) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: "Graveyard".to_owned(),
            styles: PageCss::Graveyard,
            scripts,
            date_format: date_format.to_owned(),
        }
    }
}

impl From<&[GraveyardPlant]> for Graveyard {
    fn from(graveyard: &[GraveyardPlant]) -> Graveyard {
        Graveyard {
            graveyard_table: GraveyardTable::from(graveyard),
        }
    }
}
