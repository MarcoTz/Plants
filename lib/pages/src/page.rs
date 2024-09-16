use super::shared::{footer::Footer, header::Header, html_head::HtmlHead};
use html::{
    attribute::Attribute,
    css::CssDocument,
    elements::{Body, Head, HtmlElement},
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
    fn get_title(&self) -> String;
    fn get_head(&self, date_format: &str) -> HtmlHead;
    fn get_content(&self, date_format: &str) -> HtmlElement;
    fn get_footer(&self, num_plants: i32) -> Footer {
        Footer::from(num_plants)
    }
    fn get_header(&self, relative_up: bool) -> Header {
        Header::from(relative_up)
    }

    fn render(&self, date_format: &str, relative_up: bool, num_plants: i32) -> HtmlDocument {
        log::info!("rendering {} with default implementation", self.get_title());
        let body_contents: HtmlElement = vec![
            self.get_header(relative_up).render(date_format),
            self.get_content(date_format),
            self.get_footer(num_plants).render(date_format),
        ]
        .into();
        HtmlDocument {
            head: Head::from(&self.get_head(date_format)),
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

#[cfg(test)]
mod page_tests {
    use super::{Page, PageComponent, PageURLs};
    use crate::{
        graveyard::Graveyard,
        shared::{footer::Footer, header::Header},
        test_common::{example_graveyard_plant1, example_graveyard_plant2, DATE_FORMAT},
    };
    use html::{
        attribute::Attribute,
        elements::{Body, Head},
        html_document::HtmlDocument,
    };
    use std::rc::Rc;

    fn example_graveyard() -> Graveyard {
        Graveyard::from(vec![example_graveyard_plant1(), example_graveyard_plant2()].as_slice())
    }

    #[test]
    fn graveyard_footer() {
        let result = example_graveyard().get_footer(2);
        let expected = Footer::from(2);
        assert_eq!(result, expected)
    }

    #[test]
    fn graveyard_header() {
        let result = example_graveyard().get_header(false);
        let expected = Header::from(false);
        assert_eq!(result, expected)
    }

    #[test]
    fn graveyard_render() {
        let result = example_graveyard().render(DATE_FORMAT, false, 2);
        let expected = HtmlDocument {
            head: Head::from(&example_graveyard().get_head(DATE_FORMAT)),
            body: Body {
                attributes: vec![Attribute::OnLoad("setup_img_events()".to_owned())],
                content: Rc::new(
                    vec![
                        Header::from(false).render(DATE_FORMAT),
                        example_graveyard().get_content(DATE_FORMAT),
                        Footer::from(2).render(DATE_FORMAT),
                    ]
                    .into(),
                ),
            },
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn get_index_url() {
        let result = PageURLs::IndexUrl.get_url();
        let expected = "index.html";
        assert_eq!(result, expected)
    }

    #[test]
    fn get_plant_overview_url() {
        let result = PageURLs::PlantsOverviewUrl.get_url();
        let expected = "plant_overview.html";
        assert_eq!(result, expected)
    }

    #[test]
    fn get_species_overview_url() {
        let result = PageURLs::SpeciesOverviewUrl.get_url();
        let expected = "species_overview.html";
        assert_eq!(result, expected)
    }

    #[test]
    fn get_gallery_url() {
        let result = PageURLs::GalleryUrl.get_url();
        let expected = "gallery.html";
        assert_eq!(result, expected)
    }

    #[test]
    fn get_activities_url() {
        let result = PageURLs::ActivitiesUrl.get_url();
        let expected = "activities.html";
        assert_eq!(result, expected)
    }

    #[test]
    fn get_graveyard_url() {
        let result = PageURLs::GraveyardUrl.get_url();
        let expected = "graveyard.html";
        assert_eq!(result, expected)
    }
}
