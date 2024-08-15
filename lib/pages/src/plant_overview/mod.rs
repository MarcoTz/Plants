pub mod plant_list;
pub mod plant_search;

use crate::shared::html_head::HtmlHead;
use crate::{
    css::DefinedDocument,
    errors::Error,
    page::{Page, PageComponent},
};
use html::elements::HtmlElement;
use plant_list::PlantList;
use plant_search::PlantSearch;
use plants::plant::Plant;

pub struct PlantOverview {
    pub search: PlantSearch,
    pub plant_list: PlantList,
}

impl Page for PlantOverview {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        vec![
            self.search.render(date_format),
            self.plant_list.render(date_format),
        ]
        .into()
    }

    fn get_head(&self) -> HtmlHead {
        let styles_extern = vec!["css/plant_list.css".to_owned()];
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: "Plants".to_owned(),
            styles_extern,
            styles: vec![
                DefinedDocument::Main,
                DefinedDocument::Header,
                DefinedDocument::Footer,
                DefinedDocument::PlantSearch,
            ],
            scripts,
        }
    }
}

impl TryFrom<&[Plant]> for PlantOverview {
    type Error = Error;
    fn try_from(plants: &[Plant]) -> Result<PlantOverview, Self::Error> {
        let plant_list = PlantList::try_from(plants)?;
        Ok(PlantOverview {
            search: PlantSearch {},
            plant_list,
        })
    }
}
