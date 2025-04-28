use crate::page::PageComponent;
use chrono::NaiveDate;
use html::{
    attribute::Attribute,
    elements::{Div, Figure, HeaderSize, Headline, HtmlElement, Img, A},
};
use plants::{
    named::Named,
    plant::{Plant, PlantImage},
};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlantImg {
    img_base: String,
    img_url: String,
    date: NaiveDate,
    num_images: i32,
    num_self: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlantGallery {
    pub plant_name: String,
    plant_url: String,
    plant_images: Vec<PlantImg>,
}

impl PageComponent for PlantImg {
    fn render(&self, date_format: &str) -> HtmlElement {
        let caption = vec![
            Div {
                attributes: vec![Attribute::Class(vec!["img_date".to_owned()])],
                content: Rc::new(self.date.format(date_format).to_string().into()),
            }
            .into(),
            Div {
                attributes: vec![Attribute::Class(vec!["img_nr".to_owned()])],
                content: {
                    let num_total_str = self.num_images.to_string();
                    let num_self_str = self.num_self.to_string();
                    Rc::new(format!("{num_self_str}/{num_total_str}").into())
                },
            }
            .into(),
        ];
        let mut img_path = self.img_base.to_owned();
        img_path.push_str(&self.img_url);
        let visible = if self.num_self == 1 {
            "display:block;".to_owned()
        } else {
            "display:none;".to_owned()
        };
        Figure {
            attributes: vec![
                Attribute::Class(vec!["plant_image".to_owned()]),
                Attribute::Style(visible),
            ],
            content: Rc::new(
                Img {
                    attributes: vec![Attribute::Src(img_path)],
                }
                .into(),
            ),
            caption: Rc::new(caption.into()),
        }
        .into()
    }
}

impl PageComponent for PlantGallery {
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut images_sorted = self.plant_images.clone();
        images_sorted.sort_by(|img1, img2| img2.num_self.cmp(&img1.num_self));
        let images_rendered: Vec<HtmlElement> = images_sorted
            .iter()
            .map(|x| x.render(date_format))
            .collect();
        let controls_div = Div {
            attributes: vec![Attribute::Class(vec!["img_controls".to_owned()])],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Class(vec!["left_arrow".to_owned()])],
                        content: Rc::new("&#9754;".to_owned().into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["right_arrow".to_owned()])],
                        content: Rc::new("&#9755".to_owned().into()),
                    }
                    .into(),
                ]
                .into(),
            ),
        };
        let container_content = vec![
            Headline {
                attributes: vec![],
                size: HeaderSize::H2,
                content: Rc::new(
                    A {
                        attributes: vec![Attribute::Href(self.plant_url.clone())],
                        content: Rc::new(self.plant_name.clone().into()),
                    }
                    .into(),
                ),
            }
            .into(),
            Div {
                attributes: vec![Attribute::Class(vec!["images_plant".to_owned()])],
                content: Rc::new(images_rendered.into()),
            }
            .into(),
            controls_div.into(),
        ];
        Div {
            attributes: vec![Attribute::Class(vec![
                "flex_container".to_owned(),
                "images_plant_container".to_owned(),
            ])],
            content: Rc::new(container_content.into()),
        }
        .into()
    }
}

impl From<(&Plant, &str)> for PlantGallery {
    fn from((plant, img_base): (&Plant, &str)) -> PlantGallery {
        let get_info = |x: &PlantImage, i: i32| ImageInfo {
            image: x.clone(),
            base_dir: img_base.to_owned() + "/" + &plant.get_name().replace(' ', "") + "/",
            num_images: plant.images.len() as i32,
            num_self: i,
        };
        let mut images_sorted = plant.images.clone();
        images_sorted.sort_by(|img1, img2| img2.created.cmp(&img1.created));
        PlantGallery {
            plant_name: plant.info.name.clone(),
            plant_url: plant.get_url(img_base),
            plant_images: images_sorted
                .iter()
                .enumerate()
                .map(|(i, x)| get_info(x, i as i32 + 1).into())
                .collect(),
        }
    }
}

pub struct ImageInfo {
    pub image: PlantImage,
    pub base_dir: String,
    pub num_images: i32,
    pub num_self: i32,
}
impl From<ImageInfo> for PlantImg {
    fn from(info: ImageInfo) -> PlantImg {
        PlantImg {
            img_url: info.image.file_name,
            img_base: info.base_dir.clone(),
            date: info.image.created,
            num_images: info.num_images,
            num_self: info.num_self,
        }
    }
}

#[cfg(test)]
mod plant_gallery_tests {

    use super::{ImageInfo, PageComponent, PlantGallery, PlantImage, PlantImg};
    use crate::test_common::{example_plant1, sample_date1, DATE_FORMAT};
    use html::{
        attribute::Attribute,
        elements::{Div, Figure, HeaderSize, Headline, HtmlElement, Img, A},
    };
    use std::{path::PathBuf, rc::Rc};

    fn example_gallery() -> PlantGallery {
        PlantGallery {
            plant_name: "Plant1".to_owned(),
            plant_url: "plants/Plant1.html".to_owned(),
            plant_images: vec![example_img(1), example_img(2)],
        }
    }

    fn example_image() -> PlantImage {
        PlantImage {
            created: sample_date1(),
            file_name: "img.jpg".to_owned(),
            file_path: PathBuf::from("./"),
        }
    }

    fn example_image_info(num_self: i32) -> ImageInfo {
        ImageInfo {
            image: example_image(),
            base_dir: "plants/Plant1/".to_owned(),
            num_images: 2,
            num_self,
        }
    }
    fn example_img(num_self: i32) -> PlantImg {
        PlantImg {
            img_base: "plants/Plant1/".to_owned(),
            img_url: "img.jpg".to_owned(),
            date: sample_date1(),
            num_images: 2,
            num_self,
        }
    }

    fn example_img_rendered(num_self: i32) -> HtmlElement {
        let visible = if num_self == 1 {
            "display:block;".to_owned()
        } else {
            "display:none;".to_owned()
        };
        Figure {
            attributes: vec![
                Attribute::Class(vec!["plant_image".to_owned()]),
                Attribute::Style(visible),
            ],
            content: Rc::new(
                Img {
                    attributes: vec![Attribute::Src("plants/Plant1/img.jpg".to_owned())],
                }
                .into(),
            ),
            caption: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Class(vec!["img_date".to_owned()])],
                        content: Rc::new(sample_date1().format(DATE_FORMAT).to_string().into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["img_nr".to_owned()])],
                        content: {
                            let num_total_str = "2".to_owned();
                            let num_self_str = num_self.to_string();
                            Rc::new(format!("{num_self_str}/{num_total_str}").into())
                        },
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }

    #[test]
    fn render_gallery() {
        let result = example_gallery().render(DATE_FORMAT);
        let expected = Div {
            attributes: vec![Attribute::Class(vec![
                "flex_container".to_owned(),
                "images_plant_container".to_owned(),
            ])],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H2,
                        content: Rc::new(
                            A {
                                attributes: vec![Attribute::Href("plants/Plant1.html".to_owned())],
                                content: Rc::new("Plant1".to_owned().into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["images_plant".to_owned()])],
                        content: Rc::new(
                            vec![example_img_rendered(2), example_img_rendered(1)].into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["img_controls".to_owned()])],
                        content: Rc::new(
                            vec![
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "left_arrow".to_owned()
                                    ])],
                                    content: Rc::new("&#9754;".to_owned().into()),
                                }
                                .into(),
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "right_arrow".to_owned()
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
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_image() {
        let result = example_img(1).render(DATE_FORMAT);
        let expected = example_img_rendered(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn gallery_into() {
        let mut plant = example_plant1();
        plant.images = vec![example_image(), example_image()];
        let result = PlantGallery::from((&plant, "plants"));
        let expected = example_gallery();
        assert_eq!(result, expected)
    }

    #[test]
    fn img_into() {
        let result = PlantImg::from(example_image_info(1));
        let expected = example_img(1);
        assert_eq!(result, expected)
    }
}
