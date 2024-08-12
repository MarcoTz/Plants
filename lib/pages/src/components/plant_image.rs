use super::page_component::PageComponent;
use chrono::NaiveDate;
use html::{attribute::Attribute, div::Div, figure::Figure, html_element::HtmlElement, img::Img};
use plants::plant;
use std::rc::Rc;

pub struct PlantImage {
    img_base: String,
    img_url: String,
    date: NaiveDate,
    num_images: i32,
    num_self: i32,
}

impl PageComponent for PlantImage {
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
        img_path.push_str("/");
        img_path.push_str(&self.img_url);
        Figure {
            attributes: vec![Attribute::Class(vec!["plant_image".to_owned()])],
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

pub struct ImageInfo {
    pub image: plant::PlantImage,
    pub base_dir: String,
    pub num_images: i32,
    pub num_self: i32,
}
impl From<ImageInfo> for PlantImage {
    fn from(info: ImageInfo) -> PlantImage {
        PlantImage {
            img_url: info.image.1.clone(),
            img_base: info.base_dir.clone(),
            date: info.image.0,
            num_images: info.num_images,
            num_self: info.num_self,
        }
    }
}
