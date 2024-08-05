use super::super::{
    super::html_components::{component::HtmlComponent, div::Div, figure::Figure, img::Img},
    page::PageComponent,
};
use chrono::NaiveDate;
use std::rc::Rc;

pub struct PlantImage {
    img_url: String,
    date: NaiveDate,
    date_format: String,
    num_images: i32,
    num_self: i32,
}

impl PageComponent for PlantImage {
    fn render(&self) -> HtmlComponent {
        let caption = vec![
            Div {
                class: Some("img_date".to_owned()),
                id: None,
                content: Rc::new(self.date.format(&self.date_format).to_string().into()),
            }
            .into(),
            Div {
                class: Some("img_nr".to_owned()),
                id: None,
                content: {
                    let num_total_str = self.num_images.to_string();
                    let num_self_str = self.num_self.to_string();
                    Rc::new(format!("{num_self_str}/{num_total_str}").into())
                },
            }
            .into(),
        ];
        Figure {
            class: Some("plant_image".to_owned()),
            content: Rc::new(
                Img {
                    id: None,
                    style: None,
                    src: self.img_url.clone(),
                }
                .into(),
            ),
            caption: Rc::new(caption.into()),
        }
        .into()
    }
}
