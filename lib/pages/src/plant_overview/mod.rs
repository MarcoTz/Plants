pub mod plant_list;
pub mod plant_search;

use crate::shared::html_head::HtmlHead;
use crate::{
    css::PageCss,
    page::{Page, PageComponent},
};
use html::elements::HtmlElement;
use plant_list::PlantList;
use plant_search::PlantSearch;
use plants::plant::Plant;

#[derive(Debug, PartialEq)]
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

impl From<&[Plant]> for PlantOverview {
    fn from(plants: &[Plant]) -> PlantOverview {
        log::info!("loading plant overview");
        let plant_list = PlantList::from(plants);
        PlantOverview {
            search: PlantSearch {},
            plant_list,
        }
    }
}

#[cfg(test)]
mod plant_overview_tests {
    use super::{HtmlHead, Page, PageComponent, PageCss, PlantList, PlantOverview, PlantSearch};
    use crate::test_common::{example_plant1, example_plant2, example_plant3, DATE_FORMAT};

    fn example_overview() -> PlantOverview {
        PlantOverview {
            search: PlantSearch {},
            plant_list: PlantList::from(
                vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
            ),
        }
    }

    #[test]
    fn overview_title() {
        let result = example_overview().get_title();
        let expected = "Plants".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn overview_content() {
        let result = example_overview().get_content(DATE_FORMAT);
        let expected = vec![
            PlantSearch {}.render(DATE_FORMAT),
            PlantList::from(vec![example_plant1(), example_plant2(), example_plant3()].as_slice())
                .render(DATE_FORMAT),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn overview_head() {
        let result = example_overview().get_head(DATE_FORMAT);
        let expected = HtmlHead {
            title: "Plants".to_owned(),
            styles: PageCss::PlantOverview,
            scripts: vec!["js/main.js".to_owned(), "js/plant_filter.js".to_owned()],
            date_format: DATE_FORMAT.to_owned(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn overview_into() {
        let result = PlantOverview::from(
            vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
        );
        let expected = example_overview();
        assert_eq!(result, expected)
    }
}
