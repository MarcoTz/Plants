use super::{
    components::{page_component::PageComponent, plant_gallery::PlantGallery},
    page::Page,
    shared::html_head::HtmlHead,
};
use html::{attribute::Attribute, div::Div, html_element::HtmlElement};
use plants::plant::Plant;
use std::rc::Rc;

pub struct Gallery {
    pub plant_galleries: Vec<PlantGallery>,
}

impl Page for Gallery {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        let galleries_rendered: Vec<HtmlElement> = self
            .plant_galleries
            .iter()
            .map(|x| x.render(date_format))
            .collect();
        Div {
            attributes: vec![Attribute::Id("plant_gallery".to_owned())],
            content: Rc::new(galleries_rendered.into()),
        }
        .into()
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "css/main.css".to_owned(),
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
        ];
        HtmlHead {
            title: "Gallery".to_owned(),
            styles,
        }
    }
}

impl From<&[Plant]> for Gallery {
    fn from(plants: &[Plant]) -> Gallery {
        let img_base = "img/plants";
        let plant_galleries = plants.iter().map(|x| (x, img_base).into()).collect();
        Gallery { plant_galleries }
    }
}
