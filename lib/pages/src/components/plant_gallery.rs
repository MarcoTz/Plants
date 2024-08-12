use super::{
    page_component::PageComponent,
    plant_image::{ImageInfo, PlantImage},
};
use html::{
    a::A,
    attribute::Attribute,
    div::Div,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
};
use plants::plant::Plant;

use std::rc::Rc;

pub struct PlantGallery {
    plant_name: String,
    plant_url: String,
    plant_images: Vec<PlantImage>,
}

impl PageComponent for PlantGallery {
    fn render(&self, date_format: &str) -> HtmlElement {
        let images_rendered: Vec<HtmlElement> = self
            .plant_images
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
            attributes: vec![Attribute::Class(vec!["images_plant_container".to_owned()])],
            content: Rc::new(container_content.into()),
        }
        .into()
    }
}

impl From<(&Plant, &str)> for PlantGallery {
    fn from((plant, img_base): (&Plant, &str)) -> PlantGallery {
        let get_info = |x: &plants::plant::PlantImage, i: i32| ImageInfo {
            image: x.clone(),
            base_dir: img_base.to_owned(),
            num_images: plant.images.len() as i32,
            num_self: i,
        };
        PlantGallery {
            plant_name: plant.name.clone(),
            plant_url: plant.get_url("plants/"),
            plant_images: plant
                .images
                .iter()
                .enumerate()
                .map(|(i, x)| get_info(x, i as i32).into())
                .collect(),
        }
    }
}
