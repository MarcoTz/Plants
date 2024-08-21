use super::{
    css::PageCss,
    page::Page,
    page::PageComponent,
    shared::{html_head::HtmlHead, plant_gallery::PlantGallery},
};
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement},
};
use log;
use plants::plant::Plant;
use std::rc::Rc;

pub struct Gallery {
    pub plant_galleries: Vec<PlantGallery>,
}

impl Page for Gallery {
    fn get_title(&self) -> String {
        "Gallery".to_owned()
    }

    fn get_content(&self, date_format: &str) -> HtmlElement {
        log::info!("Loading Gallery Html");
        let mut galleries_sorted = self.plant_galleries.clone();
        galleries_sorted
            .sort_by(|gallery1, gallery2| gallery1.plant_name.cmp(&gallery2.plant_name));
        let galleries_rendered: Vec<HtmlElement> = galleries_sorted
            .iter()
            .map(|x| x.render(date_format))
            .collect();
        Div {
            attributes: vec![
                Attribute::Id("plant_gallery".to_owned()),
                Attribute::Class(vec![
                    "flex_container".to_owned(),
                    "alternating_children".to_owned(),
                ]),
            ],
            content: Rc::new(galleries_rendered.into()),
        }
        .into()
    }

    fn get_head(&self, date_format: &str) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: self.get_title(),
            styles: PageCss::Gallery,
            scripts,
            date_format: date_format.to_owned(),
        }
    }
}

impl From<&[Plant]> for Gallery {
    fn from(plants: &[Plant]) -> Gallery {
        log::info!("Getting Plant Galleries");
        let img_base = "img/plants";
        let plant_galleries = plants.iter().map(|x| (x, img_base).into()).collect();
        Gallery { plant_galleries }
    }
}
