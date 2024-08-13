pub mod autowatered;
pub mod hall_of_fame;
pub mod upcoming_tasks;

use super::{
    errors::Error,
    index::{autowatered::AutoWatered, hall_of_fame::HallOfFame, upcoming_tasks::UpcomingTasks},
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use html::html_element::HtmlElement;
use plants::plant::Plant;

pub struct Index {
    pub next_activities: UpcomingTasks,
    pub autowatered: AutoWatered,
    pub hall_of_fame: HallOfFame,
}

impl Page for Index {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        vec![
            self.next_activities.render(date_format),
            self.autowatered.render(date_format),
            self.hall_of_fame.render(date_format),
        ]
        .into()
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "css/main.css".to_owned(),
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
            "css/upcoming_tasks.css".to_owned(),
            "css/hall_of_fame.css".to_owned(),
        ];
        HtmlHead {
            title: "Dashboard".to_owned(),
            styles,
        }
    }
}

impl TryFrom<&[Plant]> for Index {
    type Error = Error;
    fn try_from(plants: &[Plant]) -> Result<Index, Self::Error> {
        let hall_of_fame = HallOfFame::try_from(plants)?;
        Ok(Index {
            next_activities: UpcomingTasks::from(plants),
            autowatered: AutoWatered::from(plants),
            hall_of_fame,
        })
    }
}
