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
