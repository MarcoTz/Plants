use crate::page::PageComponent;
use chrono::NaiveDate;
use html::{
    a::A,
    attribute::Attribute,
    div::Div,
    figure::Figure,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
    img::Img,
};
use plants::plant::{Plant, PlantImage};
use std::rc::Rc;

#[derive(Clone)]
pub struct PlantImg {
    img_base: String,
    img_url: String,
    date: NaiveDate,
    num_images: i32,
    num_self: i32,
}

#[derive(Clone)]
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
        img_path.push('/');
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
            base_dir: img_base.to_owned(),
            num_images: plant.images.len() as i32,
            num_self: i,
        };
        let mut images_sorted = plant.images.clone();
        images_sorted.sort_by(|img1, img2| img1.0.cmp(&img2.0));
        PlantGallery {
            plant_name: plant.name.clone(),
            plant_url: plant.get_url("plants"),
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
            img_url: info.image.1.clone(),
            img_base: info.base_dir.clone(),
            date: info.image.0,
            num_images: info.num_images,
            num_self: info.num_self,
        }
    }
}
