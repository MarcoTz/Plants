use super::page_component::PageComponent;
use chrono::NaiveDate;
use html::{attribute::Attribute, div::Div, figure::Figure, html_element::HtmlElement, img::Img};
use plants::plant;
use std::rc::Rc;

pub struct PlantImage {
    img_url: String,
    date: NaiveDate,
    num_images: i32,
    num_self: i32,
}

impl PageComponent for PlantImage {
    fn render(&self, date_format: &str) -> HtmlElement {
        let caption = vec![
            Div {
                attributes: vec![Attribute::Class("img_date".to_owned())],
                content: Rc::new(self.date.format(date_format).to_string().into()),
            }
            .into(),
            Div {
                attributes: vec![Attribute::Class("img_nr".to_owned())],
                content: {
                    let num_total_str = self.num_images.to_string();
                    let num_self_str = self.num_self.to_string();
                    Rc::new(format!("{num_self_str}/{num_total_str}").into())
                },
            }
            .into(),
        ];
        Figure {
            attributes: vec![Attribute::Class("plant_image".to_owned())],
            content: Rc::new(
                Img {
                    attributes: vec![Attribute::Src(self.img_url.clone())],
                }
                .into(),
            ),
            caption: Rc::new(caption.into()),
        }
        .into()
    }
}

impl From<&plant::PlantImage> for PlantImage {
    fn from(plant_img: &plant::PlantImage) -> PlantImage {
        PlantImage {
            img_url: plant_img.1.clone(),
            date: plant_img.0,
            num_images: 0,
            num_self: 0,
        }
    }
}
