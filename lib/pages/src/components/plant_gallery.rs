use super::{page_component::PageComponent, plant_image::PlantImage};
use html::{
    a::A,
    attribute::Attribute,
    div::Div,
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
};

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
            attributes: vec![Attribute::Class("img_controls".to_owned())],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Class("left_arrow".to_owned())],
                        content: Rc::new("&#9754;".to_owned().into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class("right_arrow".to_owned())],
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
                attributes: vec![Attribute::Class("images_plant".to_owned())],
                content: Rc::new(images_rendered.into()),
            }
            .into(),
            controls_div.into(),
        ];
        Div {
            attributes: vec![Attribute::Class("images_plant_container".to_owned())],
            content: Rc::new(container_content.into()),
        }
        .into()
    }
}
