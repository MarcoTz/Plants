use super::{
    components::page_component::PageComponent,
    page::Page,
    shared::{html_head::HtmlHead, plant_gallery::PlantGallery},
};
use html::{attribute::Attribute, div::Div, html_element::HtmlElement};
use plants::plant::Plant;
use std::rc::Rc;

pub struct Gallery {
    pub plant_galleries: Vec<PlantGallery>,
}

impl Page for Gallery {
    fn get_content(&self, date_format: &str) -> HtmlElement {
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

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "css/main.css".to_owned(),
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
            "css/gallery.css".to_owned(),
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
