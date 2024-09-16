pub mod autowatered;
pub mod hall_of_fame;
pub mod upcoming_tasks;

use super::{
    css::PageCss,
    errors::Error,
    index::{autowatered::AutoWatered, hall_of_fame::HallOfFame, upcoming_tasks::UpcomingTasks},
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use html::elements::HtmlElement;
use plants::plant::Plant;

#[derive(Debug, PartialEq)]
pub struct Index {
    pub next_activities: UpcomingTasks,
    pub autowatered: AutoWatered,
    pub hall_of_fame: HallOfFame,
}

impl Page for Index {
    fn get_title(&self) -> String {
        "Dashboard".to_owned()
    }

    fn get_content(&self, date_format: &str) -> HtmlElement {
        vec![
            self.next_activities.render(date_format),
            self.autowatered.render(date_format),
            self.hall_of_fame.render(date_format),
        ]
        .into()
    }

    fn get_head(&self, date_format: &str) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: self.get_title(),
            styles: PageCss::Index,
            scripts,
            date_format: date_format.to_owned(),
        }
    }
}

impl TryFrom<&[Plant]> for Index {
    type Error = Error;
    fn try_from(plants: &[Plant]) -> Result<Index, Self::Error> {
        log::info!("Loading Index");
        let hall_of_fame = HallOfFame::try_from(plants)?;
        Ok(Index {
            next_activities: UpcomingTasks::from(plants),
            autowatered: AutoWatered::from(plants),
            hall_of_fame,
        })
    }
}

#[cfg(test)]
mod index_tests {
    use super::{
        AutoWatered, HallOfFame, HtmlHead, Index, Page, PageComponent, PageCss, UpcomingTasks,
    };
    use crate::test_common::{example_plant1, example_plant2, example_plant3, DATE_FORMAT};
    use plants::plant::Plant;

    fn example_index() -> Index {
        Index {
            next_activities: UpcomingTasks::from(example_plants().as_slice()),
            autowatered: AutoWatered::from(example_plants().as_slice()),
            hall_of_fame: HallOfFame::try_from(example_plants().as_slice()).unwrap(),
        }
    }

    fn example_plants() -> Vec<Plant> {
        vec![example_plant1(), example_plant2(), example_plant3()]
    }

    #[test]
    fn index_title() {
        let result = example_index().get_title();
        let expected = "Dashboard".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn index_content() {
        let result = example_index().get_content(DATE_FORMAT);
        let expected = vec![
            UpcomingTasks::from(example_plants().as_slice()).render(DATE_FORMAT),
            AutoWatered::from(example_plants().as_slice()).render(DATE_FORMAT),
            HallOfFame::try_from(example_plants().as_slice())
                .unwrap()
                .render(DATE_FORMAT),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn index_head() {
        let result = example_index().get_head(DATE_FORMAT);
        let expected = HtmlHead {
            title: "Dashboard".to_owned(),
            styles: PageCss::Index,
            scripts: vec!["js/main.js".to_owned()],
            date_format: DATE_FORMAT.to_owned(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn index_into() {
        let result = Index::try_from(example_plants().as_slice()).unwrap();
        let expected = example_index();
        assert_eq!(result, expected)
    }

    #[test]
    fn index_into_fail() {
        let mut plant = example_plant1();
        plant.growth = vec![];
        let result = Index::try_from(vec![plant].as_slice());
        assert!(result.is_err())
    }
}
