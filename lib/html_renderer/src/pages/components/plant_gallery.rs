use super::{
    super::{
        super::html_components::{
            component::HtmlComponent,
            div::Div,
            headline::{HeaderSize, Headline},
            link::Link,
        },
        page::PageComponent,
    },
    plant_image::PlantImage,
};

use std::rc::Rc;

pub struct PlantGallery {
    plant_name: String,
    plant_url: String,
    plant_images: Vec<PlantImage>,
}

impl PageComponent for PlantGallery {
    fn render(&self) -> HtmlComponent {
        let images_rendered: Vec<HtmlComponent> =
            self.plant_images.iter().map(|x| x.render()).collect();
        let controls_div = Div {
            class: Some("img_controls".to_owned()),
            id: None,
            contents: Rc::new(
                vec![
                    Div {
                        id: None,
                        class: Some("left_arrow".to_owned()),
                        contents: Rc::new("&#9754;".to_owned().into()),
                    }
                    .into(),
                    Div {
                        id: None,
                        class: Some("right_arrow".to_owned()),
                        contents: Rc::new("&#9755".to_owned().into()),
                    }
                    .into(),
                ]
                .into(),
            ),
        };
        let container_contents = vec![
            Headline {
                size: HeaderSize::H2,
                contents: Rc::new(
                    Link {
                        href: self.plant_url.clone(),
                        contents: Rc::new(self.plant_name.clone().into()),
                    }
                    .into(),
                ),
            }
            .into(),
            Div {
                class: Some("images_plant".to_owned()),
                id: None,
                contents: Rc::new(images_rendered.into()),
            }
            .into(),
            controls_div.into(),
        ];
        Div {
            id: None,
            class: Some("images_plant_container".to_owned()),
            contents: Rc::new(container_contents.into()),
        }
        .into()
    }
}
