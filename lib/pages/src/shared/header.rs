use crate::page::{PageComponent, PageURLs};
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, A},
};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub dashboard_url: String,
    pub plants_url: String,
    pub species_url: String,
    pub gallery_url: String,
    pub activities_url: String,
    pub graveyard_url: String,
}

impl PageComponent for Header {
    fn render(&self, _: &str) -> HtmlElement {
        let links: Vec<HtmlElement> = vec![
            A {
                attributes: vec![Attribute::Href(self.dashboard_url.clone())],
                content: Rc::new("Dashboard".to_owned().into()),
            }
            .into(),
            A {
                attributes: vec![Attribute::Href(self.plants_url.clone())],
                content: Rc::new("Plants".to_owned().into()),
            }
            .into(),
            A {
                attributes: vec![Attribute::Href(self.species_url.clone())],
                content: Rc::new("Species".to_owned().into()),
            }
            .into(),
            A {
                attributes: vec![Attribute::Href(self.gallery_url.clone())],
                content: Rc::new("Gallery".to_owned().into()),
            }
            .into(),
            A {
                attributes: vec![Attribute::Href(self.activities_url.clone())],
                content: Rc::new("Activities".to_owned().into()),
            }
            .into(),
            A {
                attributes: vec![Attribute::Href(self.graveyard_url.clone())],
                content: Rc::new("Graveyard".to_owned().into()),
            }
            .into(),
        ];
        Div {
            attributes: vec![
                Attribute::Class(vec![
                    "alternating_children".to_owned(),
                    "flex_container".to_owned(),
                ]),
                Attribute::Id("header".to_owned()),
            ],
            content: Rc::new(links.into()),
        }
        .into()
    }
}

impl From<bool> for Header {
    fn from(relative_up: bool) -> Header {
        let prefix = if relative_up {
            "../".to_owned()
        } else {
            "./".to_owned()
        };
        Header {
            dashboard_url: prefix.clone() + &PageURLs::get_url(PageURLs::IndexUrl),
            plants_url: prefix.clone() + &PageURLs::get_url(PageURLs::PlantsOverviewUrl),
            species_url: prefix.clone() + &PageURLs::get_url(PageURLs::SpeciesOverviewUrl),
            gallery_url: prefix.clone() + &PageURLs::get_url(PageURLs::GalleryUrl),
            activities_url: prefix.clone() + &PageURLs::get_url(PageURLs::ActivitiesUrl),
            graveyard_url: prefix.clone() + &PageURLs::get_url(PageURLs::GraveyardUrl),
        }
    }
}

#[cfg(test)]
mod header_tests {
    use super::{Header, PageComponent, PageURLs};
    use crate::test_common::DATE_FORMAT;
    use html::{
        attribute::Attribute,
        elements::{Div, HtmlElement, A},
    };
    use std::rc::Rc;

    fn example_header_true() -> Header {
        Header {
            dashboard_url: "../".to_owned() + &PageURLs::get_url(PageURLs::IndexUrl),
            plants_url: "../".to_owned() + &PageURLs::get_url(PageURLs::PlantsOverviewUrl),
            species_url: "../".to_owned() + &PageURLs::get_url(PageURLs::SpeciesOverviewUrl),
            gallery_url: "../".to_owned() + &PageURLs::get_url(PageURLs::GalleryUrl),
            activities_url: "../".to_owned() + &PageURLs::get_url(PageURLs::ActivitiesUrl),
            graveyard_url: "../".to_owned() + &PageURLs::get_url(PageURLs::GraveyardUrl),
        }
    }
    fn example_header_false() -> Header {
        Header {
            dashboard_url: "./".to_owned() + &PageURLs::get_url(PageURLs::IndexUrl),
            plants_url: "./".to_owned() + &PageURLs::get_url(PageURLs::PlantsOverviewUrl),
            species_url: "./".to_owned() + &PageURLs::get_url(PageURLs::SpeciesOverviewUrl),
            gallery_url: "./".to_owned() + &PageURLs::get_url(PageURLs::GalleryUrl),
            activities_url: "./".to_owned() + &PageURLs::get_url(PageURLs::ActivitiesUrl),
            graveyard_url: "./".to_owned() + &PageURLs::get_url(PageURLs::GraveyardUrl),
        }
    }

    fn example_header_rendered(relative_up: bool) -> HtmlElement {
        let header = if relative_up {
            example_header_true()
        } else {
            example_header_false()
        };
        Div {
            attributes: vec![
                Attribute::Class(vec![
                    "alternating_children".to_owned(),
                    "flex_container".to_owned(),
                ]),
                Attribute::Id("header".to_owned()),
            ],
            content: Rc::new(
                vec![
                    A {
                        attributes: vec![Attribute::Href(header.dashboard_url)],
                        content: Rc::new("Dashboard".to_owned().into()),
                    }
                    .into(),
                    A {
                        attributes: vec![Attribute::Href(header.plants_url)],
                        content: Rc::new("Plants".to_owned().into()),
                    }
                    .into(),
                    A {
                        attributes: vec![Attribute::Href(header.species_url)],
                        content: Rc::new("Species".to_owned().into()),
                    }
                    .into(),
                    A {
                        attributes: vec![Attribute::Href(header.gallery_url)],
                        content: Rc::new("Gallery".to_owned().into()),
                    }
                    .into(),
                    A {
                        attributes: vec![Attribute::Href(header.activities_url)],
                        content: Rc::new("Activities".to_owned().into()),
                    }
                    .into(),
                    A {
                        attributes: vec![Attribute::Href(header.graveyard_url)],
                        content: Rc::new("Graveyard".to_owned().into()),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }

    #[test]
    fn render_header_true() {
        let result = example_header_true().render(DATE_FORMAT);
        let expected = example_header_rendered(true);
        assert_eq!(result, expected)
    }

    #[test]
    fn render_header_false() {
        let result = example_header_false().render(DATE_FORMAT);
        let expected = example_header_rendered(false);
        assert_eq!(result, expected)
    }

    #[test]
    fn header_from_true() {
        let result = Header::from(true);
        let expected = example_header_true();
        assert_eq!(result, expected)
    }

    #[test]
    fn header_from_false() {
        let result = Header::from(false);
        let expected = example_header_false();
        assert_eq!(result, expected)
    }
}
