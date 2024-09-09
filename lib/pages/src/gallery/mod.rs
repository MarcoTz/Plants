use super::{
    css::PageCss,
    page::Page,
    page::PageComponent,
    shared::{html_head::HtmlHead, plant_gallery::PlantGallery},
};
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement},
};
use log;
use plants::plant::Plant;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Gallery {
    pub plant_galleries: Vec<PlantGallery>,
}

impl Page for Gallery {
    fn get_title(&self) -> String {
        "Gallery".to_owned()
    }

    fn get_content(&self, date_format: &str) -> HtmlElement {
        log::info!("Loading Gallery Html");
        let mut galleries_sorted = self.plant_galleries.clone();
        galleries_sorted
            .sort_by(|gallery1, gallery2| gallery1.plant_name.cmp(&gallery2.plant_name));
        let galleries_rendered: Vec<HtmlElement> = galleries_sorted
            .iter()
            .map(|x| x.render(date_format))
            .collect();
        Div {
            attributes: vec![
                Attribute::Id("plant_gallery".to_owned()),
                Attribute::Class(vec![
                    "flex_container".to_owned(),
                    "alternating_children".to_owned(),
                ]),
            ],
            content: Rc::new(galleries_rendered.into()),
        }
        .into()
    }

    fn get_head(&self, date_format: &str) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: self.get_title(),
            styles: PageCss::Gallery,
            scripts,
            date_format: date_format.to_owned(),
        }
    }
}

impl From<&[Plant]> for Gallery {
    fn from(plants: &[Plant]) -> Gallery {
        log::info!("Getting Plant Galleries");
        let img_base = "img/plants";
        let plant_galleries = plants.iter().map(|x| (x, img_base).into()).collect();
        Gallery { plant_galleries }
    }
}

#[cfg(test)]
mod gallery_tests {

    use super::{Gallery, HtmlHead, Page, PageCss, PlantGallery};
    use crate::test_common::{example_plant1, example_plant2, example_plant3, DATE_FORMAT};
    use html::{
        attribute::Attribute,
        elements::{Div, HeaderSize, Headline, A},
    };
    use std::rc::Rc;

    fn example_gallery() -> Gallery {
        Gallery {
            plant_galleries: vec![
                PlantGallery::from((&example_plant1(), DATE_FORMAT)),
                PlantGallery::from((&example_plant2(), DATE_FORMAT)),
                PlantGallery::from((&example_plant3(), DATE_FORMAT)),
            ],
        }
    }

    #[test]
    fn gallery_title() {
        let result = example_gallery().get_title();
        let expected = "Gallery".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn gallery_content() {
        let result = example_gallery().get_content(DATE_FORMAT);
        let expected = Div {
            attributes: vec![
                Attribute::Id("plant_gallery".to_owned()),
                Attribute::Class(vec![
                    "flex_container".to_owned(),
                    "alternating_children".to_owned(),
                ]),
            ],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Class(vec![
                            "flex_container".to_owned(),
                            "images_plant_container".to_owned(),
                        ])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    size: HeaderSize::H2,
                                    attributes: vec![],
                                    content: Rc::new(
                                        A {
                                            attributes: vec![Attribute::Href(
                                                "plants/Plant1.html".to_owned(),
                                            )],
                                            content: Rc::new("Plant1".to_owned().into()),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "images_plant".to_owned()
                                    ])],
                                    content: Rc::new(vec![].into()),
                                }
                                .into(),
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "img_controls".to_owned()
                                    ])],
                                    content: Rc::new(
                                        vec![
                                            Div {
                                                attributes: vec![Attribute::Class(vec![
                                                    "left_arrow".to_owned(),
                                                ])],
                                                content: Rc::new("&#9754;".to_owned().into()),
                                            }
                                            .into(),
                                            Div {
                                                attributes: vec![Attribute::Class(vec![
                                                    "right_arrow".to_owned(),
                                                ])],
                                                content: Rc::new("&#9755".to_owned().into()),
                                            }
                                            .into(),
                                        ]
                                        .into(),
                                    ),
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec![
                            "flex_container".to_owned(),
                            "images_plant_container".to_owned(),
                        ])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    size: HeaderSize::H2,
                                    attributes: vec![],
                                    content: Rc::new(
                                        A {
                                            attributes: vec![Attribute::Href(
                                                "plants/Plant2.html".to_owned(),
                                            )],
                                            content: Rc::new("Plant2".to_owned().into()),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "images_plant".to_owned()
                                    ])],
                                    content: Rc::new(vec![].into()),
                                }
                                .into(),
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "img_controls".to_owned()
                                    ])],
                                    content: Rc::new(
                                        vec![
                                            Div {
                                                attributes: vec![Attribute::Class(vec![
                                                    "left_arrow".to_owned(),
                                                ])],
                                                content: Rc::new("&#9754;".to_owned().into()),
                                            }
                                            .into(),
                                            Div {
                                                attributes: vec![Attribute::Class(vec![
                                                    "right_arrow".to_owned(),
                                                ])],
                                                content: Rc::new("&#9755".to_owned().into()),
                                            }
                                            .into(),
                                        ]
                                        .into(),
                                    ),
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec![
                            "flex_container".to_owned(),
                            "images_plant_container".to_owned(),
                        ])],
                        content: Rc::new(
                            vec![
                                Headline {
                                    size: HeaderSize::H2,
                                    attributes: vec![],
                                    content: Rc::new(
                                        A {
                                            attributes: vec![Attribute::Href(
                                                "plants/Plant3.html".to_owned(),
                                            )],
                                            content: Rc::new("Plant3".to_owned().into()),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "images_plant".to_owned()
                                    ])],
                                    content: Rc::new(vec![].into()),
                                }
                                .into(),
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "img_controls".to_owned()
                                    ])],
                                    content: Rc::new(
                                        vec![
                                            Div {
                                                attributes: vec![Attribute::Class(vec![
                                                    "left_arrow".to_owned(),
                                                ])],
                                                content: Rc::new("&#9754;".to_owned().into()),
                                            }
                                            .into(),
                                            Div {
                                                attributes: vec![Attribute::Class(vec![
                                                    "right_arrow".to_owned(),
                                                ])],
                                                content: Rc::new("&#9755".to_owned().into()),
                                            }
                                            .into(),
                                        ]
                                        .into(),
                                    ),
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn gallery_head() {
        let result = example_gallery().get_head(DATE_FORMAT);
        let expected = HtmlHead {
            title: "Gallery".to_owned(),
            styles: PageCss::Gallery,
            scripts: vec!["js/main.js".to_owned()],
            date_format: "%d.%m.%Y".to_owned(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn gallery_into() {
        let result =
            Gallery::from(vec![example_plant1(), example_plant2(), example_plant3()].as_slice());
        let expected = example_gallery();
        assert_eq!(result, expected)
    }
}
