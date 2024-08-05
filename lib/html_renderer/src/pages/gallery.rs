use super::{
    super::html_components::{
        body::Body,
        component::{Html, HtmlComponent},
        div::Div,
        head::Head,
    },
    components::{footer::Footer, header::Header, plant_gallery::PlantGallery},
    page::{Page, PageComponent},
};
use std::rc::Rc;

pub struct Gallery {
    header: Header,
    footer: Footer,
    plant_galleries: Vec<PlantGallery>,
}

impl Page for Gallery {
    fn render(&self) -> Html {
        let head = Head {
            title: "Gallery".to_owned(),
        };

        let galleries_rendered: Vec<HtmlComponent> =
            self.plant_galleries.iter().map(|x| x.render()).collect();
        let body_content = vec![
            self.header.render(),
            Div {
                id: Some("plant_gallery".to_owned()),
                class: None,
                content: Rc::new(galleries_rendered.into()),
            }
            .into(),
            self.footer.render(),
        ];
        let body = Body {
            content: Rc::new(body_content.into()),
        };
        Html { head, body }
    }
}
