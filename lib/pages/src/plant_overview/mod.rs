pub mod plant_list;
pub mod plant_search;

use crate::shared::html_head::HtmlHead;
use crate::{
    css::PageCss,
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
    fn get_title(&self) -> String {
        "Plants".to_owned()
    }

    fn get_content(&self, date_format: &str) -> HtmlElement {
        vec![
            self.search.render(date_format),
            self.plant_list.render(date_format),
        ]
        .into()
    }

    fn get_head(&self, date_format: &str) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned(), "js/plant_filter.js".to_owned()];
        HtmlHead {
            title: self.get_title(),
            styles: PageCss::PlantOverview,
            scripts,
            date_format: date_format.to_owned(),
        }
    }
}

impl TryFrom<&[Plant]> for PlantOverview {
    type Error = Error;
    fn try_from(plants: &[Plant]) -> Result<PlantOverview, Self::Error> {
        log::info!("loading plant overview");
        let plant_list = PlantList::try_from(plants)?;
        Ok(PlantOverview {
            search: PlantSearch {},
            plant_list,
        })
    }
}
