pub mod autowatered;
pub mod hall_of_fame;
pub mod upcoming_tasks;

use super::{
    components::{
        footer::Footer, header::Header, html_head::HtmlHead, page_component::PageComponent,
    },
    index::{autowatered::AutoWatered, hall_of_fame::HallOfFame, upcoming_tasks::UpcomingTasks},
    page::Page,
};
use html::{body::Body, head::Head, html_document::HtmlDocument};
use std::rc::Rc;

pub struct Index {
    pub head: HtmlHead,
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
            head: Head::from(&self.head),
            body,
        }
    }
}
