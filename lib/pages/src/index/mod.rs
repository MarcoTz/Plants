pub mod autowatered;
pub mod hall_of_fame;
pub mod upcoming_tasks;

use super::{
    components::page_component::PageComponent,
    errors::Error,
    index::{autowatered::AutoWatered, hall_of_fame::HallOfFame, upcoming_tasks::UpcomingTasks},
    page::Page,
    shared::{footer::Footer, header::Header, html_head::HtmlHead},
};
use html::{body::Body, head::Head, html_document::HtmlDocument};
use plants::plant::Plant;
use std::rc::Rc;

pub struct Index {
    pub header: Header,
    pub next_activities: UpcomingTasks,
    pub autowatered: AutoWatered,
    pub hall_of_fame: HallOfFame,
    pub footer: Footer,
}

impl Page for Index {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let body_contents = vec![
            self.header.render(date_format),
            self.next_activities.render(date_format),
            self.autowatered.render(date_format),
            self.hall_of_fame.render(date_format),
            self.footer.render(date_format),
        ];

        let body = Body {
            content: Rc::new(body_contents.into()),
        };
        HtmlDocument {
            head: Head::from(&self.get_head()),
            body,
        }
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
            header: Header::from(false),
            next_activities: UpcomingTasks::from(plants),
            autowatered: AutoWatered::from(plants),
            hall_of_fame,
            footer: Footer::from(plants.len() as i32),
        })
    }
}
