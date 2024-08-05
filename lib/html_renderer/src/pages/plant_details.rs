use super::{
    super::html_components::{
        body::Body,
        component::Html,
        head::Head,
        headline::{HeaderSize, Headline},
    },
    components::{footer::Footer, header::Header, plant_contents::PlantContents},
    page::{Page, PageComponent},
};
use std::rc::Rc;

pub struct PlantDetails {
    plant_name: String,
    plant_species: String,
    header: Header,
    footer: Footer,
    plant_content: PlantContents,
}

impl Page for PlantDetails {
    fn render(&self) -> Html {
        let head = Head {
            title: self.plant_name.clone(),
        };
        let plant_header = Headline {
            size: HeaderSize::H1,
            content: {
                let plant_name_str = self.plant_name.clone();
                let plant_species_str = self.plant_species.clone();

                Rc::new(format!("{plant_name_str} {plant_species_str}").into())
            },
        }
        .into();
        let body_content = vec![
            self.header.render(),
            plant_header,
            self.plant_content.render(),
            self.footer.render(),
        ]
        .into();
        let body = Body {
            content: Rc::new(body_content),
        };
        Html { head, body }
    }
}
