use super::{
    super::html::{
        attribute::Attribute,
        body::Body,
        div::Div,
        head::Head,
        html_element::{Html, HtmlElement},
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

        let galleries_rendered: Vec<HtmlElement> =
            self.plant_galleries.iter().map(|x| x.render()).collect();
        let body_content = vec![
            self.header.render(),
            Div {
                attributes: vec![Attribute::Id("plant_gallery".to_owned())],
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
