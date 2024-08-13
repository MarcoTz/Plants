pub mod graveyard_table;

use super::{
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

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "css/main.css".to_owned(),
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
        ];
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: "Graveyard".to_owned(),
            styles,
            scripts,
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
