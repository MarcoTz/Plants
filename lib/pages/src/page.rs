use super::shared::{footer::Footer, header::Header, html_head::HtmlHead};
use html::{
    attribute::Attribute,
    css::CssDocument,
    elements::{body::Body, head::Head, HtmlElement},
    html_document::HtmlDocument,
};
use std::rc::Rc;

pub trait PageComponent {
    fn render(&self, date_format: &str) -> HtmlElement;
}

pub trait CssComponent {
    fn render(&self) -> CssDocument;
}

pub trait Page {
    fn get_head(&self) -> HtmlHead;
    fn get_content(&self, date_format: &str) -> HtmlElement;
    fn get_footer(&self, num_plants: i32) -> Footer {
        Footer::from(num_plants)
    }
    fn get_header(&self, relative_up: bool) -> Header {
        Header::from(relative_up)
    }

    fn render(&self, date_format: &str, relative_up: bool, num_plants: i32) -> HtmlDocument {
        let body_contents: HtmlElement = vec![
            self.get_header(relative_up).render(date_format),
            self.get_content(date_format),
            self.get_footer(num_plants).render(date_format),
        ]
        .into();
        HtmlDocument {
            head: Head::from(&self.get_head()),
            body: Body {
                attributes: vec![Attribute::OnLoad("setup_img_events()".to_owned())],
                content: Rc::new(body_contents),
            },
        }
    }
}

pub enum PageURLs {
    IndexUrl,
    PlantsOverviewUrl,
    SpeciesOverviewUrl,
    GalleryUrl,
    ActivitiesUrl,
    GraveyardUrl,
}
impl PageURLs {
    pub fn get_url(self) -> String {
        match self {
            PageURLs::IndexUrl => "index.html".to_owned(),
            PageURLs::PlantsOverviewUrl => "plant_overview.html".to_owned(),
            PageURLs::SpeciesOverviewUrl => "species_overview.html".to_owned(),
            PageURLs::GalleryUrl => "gallery.html".to_owned(),
            PageURLs::ActivitiesUrl => "activities.html".to_owned(),
            PageURLs::GraveyardUrl => "graveyard.html".to_owned(),
        }
    }
}
