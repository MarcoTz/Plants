use crate::page::PageComponent;
use chrono::{Local, NaiveDate};
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, Img, A},
};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Footer {
    pub num_plants: i32,
    pub last_build: NaiveDate,
}

impl PageComponent for Footer {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut footer_content: Vec<HtmlElement> = vec![];
        let github_link = Div {
            attributes: vec![Attribute::Id("github_link".to_owned())],
            content: Rc::new(
                A {
                    attributes: vec![Attribute::Href(
                        "https://github.com/MarcoTz/Plants".to_owned(),
                    )],
                    content: Rc::new("Github".to_owned().into()),
                }
                .into(),
            ),
        };
        footer_content.push(github_link.into());

        let num_plants_str = self.num_plants.to_string();
        let num_plants: Div = Div {
            attributes: vec![Attribute::Id("num_plants".to_owned())],
            content: Rc::new(format!("Number of plants: {num_plants_str}").into()),
        };
        footer_content.push(num_plants.into());

        let last_build_str = self.last_build.format(date_format);
        let last_build: Div = Div {
            attributes: vec![Attribute::Id("last_build".to_owned())],
            content: Rc::new(format!("Last build: {last_build_str}").into()),
        };
        footer_content.push(last_build.into());

        footer_content.push(
            Div {
                attributes: vec![
                    Attribute::Id("image_viewer".to_owned()),
                    Attribute::OnClick("close_image_viewer();".to_owned()),
                    Attribute::Style("display:none;".to_owned()),
                ],
                content: Rc::new(
                    Div {
                        attributes: vec![
                            Attribute::Id("image_box".to_owned()),
                            Attribute::OnClick("close_image_viewer();".to_owned()),
                        ],
                        content: Rc::new(
                            Img {
                                attributes: vec![Attribute::Id("image_viewer_image".to_owned())],
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
            }
            .into(),
        );

        Div {
            attributes: vec![Attribute::Id("footer".to_owned())],
            content: Rc::new(footer_content.into()),
        }
        .into()
    }
}

impl From<i32> for Footer {
    fn from(num_plants: i32) -> Footer {
        Footer {
            num_plants,
            last_build: Local::now().date_naive(),
        }
    }
}

#[cfg(test)]
mod footer_tests {

    use super::{Footer, PageComponent};
    use crate::test_common::DATE_FORMAT;
    use chrono::Local;
    use html::{
        attribute::Attribute,
        elements::{Div, Img, A},
    };
    use std::rc::Rc;

    fn example_footer() -> Footer {
        Footer {
            num_plants: 10,
            last_build: Local::now().date_naive(),
        }
    }

    #[test]
    fn render_footer() {
        let result = example_footer().render(DATE_FORMAT);
        let expected = Div {
            attributes: vec![Attribute::Id("footer".to_owned())],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Id("github_link".to_owned())],
                        content: Rc::new(
                            A {
                                attributes: vec![Attribute::Href(
                                    "https://github.com/MarcoTz/Plants".to_owned(),
                                )],
                                content: Rc::new("Github".to_owned().into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Id("num_plants".to_owned())],
                        content: Rc::new(format!("Number of plants: 10").into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Id("last_build".to_owned())],
                        content: Rc::new(
                            format!(
                                "Last build: {}",
                                Local::now().date_naive().format(DATE_FORMAT).to_string()
                            )
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![
                            Attribute::Id("image_viewer".to_owned()),
                            Attribute::OnClick("close_image_viewer();".to_owned()),
                            Attribute::Style("display:none;".to_owned()),
                        ],
                        content: Rc::new(
                            Div {
                                attributes: vec![
                                    Attribute::Id("image_box".to_owned()),
                                    Attribute::OnClick("close_image_viewer();".to_owned()),
                                ],
                                content: Rc::new(
                                    Img {
                                        attributes: vec![Attribute::Id(
                                            "image_viewer_image".to_owned(),
                                        )],
                                    }
                                    .into(),
                                ),
                            }
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
    fn footer_into() {
        let result = Footer::from(10);
        let expected = example_footer();
        assert_eq!(result, expected)
    }
}
