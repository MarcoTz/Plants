use super::{
    components::{
        autowatered::AutoWatered, footer::Footer, hall_of_fame::HallOfFame, header::Header,
        next_activity::NextActivity, page_component::PageComponent,
    },
    page::Page,
};
use html::{body::Body, head::Head, html_document::HtmlDocument};
use std::rc::Rc;

pub struct Index {
    header: Header,
    next_activities: NextActivity,
    autowatered: AutoWatered,
    hall_of_fame: HallOfFame,
    footer: Footer,
}

impl Page for Index {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let mut body_contents = vec![];

        let head = Head {
            title: "index".to_owned(),
        };
        body_contents.push(self.header.render(date_format));

        body_contents.push(self.next_activities.render(date_format));
        body_contents.push(self.autowatered.render(date_format));
        body_contents.push(self.hall_of_fame.render(date_format));
        body_contents.push(self.footer.render(date_format));

        let body = Body {
            content: Rc::new(body_contents.into()),
        };
        HtmlDocument { head, body }
    }
}
