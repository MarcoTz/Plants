pub mod plant_list;
pub mod plant_search;

use crate::shared::html_head::HtmlHead;
use crate::{
    errors::Error,
    page::{Page, PageComponent},
};
use html::html_element::HtmlElement;
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
        let styles = vec![
            "css/main.css".to_owned(),
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
            "css/plant_list.css".to_owned(),
            "css/plant_search.css".to_owned(),
        ];
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: "Plants".to_owned(),
            styles,
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