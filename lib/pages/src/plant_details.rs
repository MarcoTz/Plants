use super::{
    components::{page_component::PageComponent, plant_contents::PlantContents},
    errors::Error,
    page::Page,
    shared::{footer::Footer, header::Header, html_head::HtmlHead},
};
use html::{
    body::Body,
    head::Head,
    headline::{HeaderSize, Headline},
    html_document::HtmlDocument,
};
use plants::plant::Plant;
use std::rc::Rc;

pub struct PlantDetails {
    pub plant_name: String,
    pub plant_species: Option<String>,
    pub header: Header,
    pub footer: Footer,
    pub plant_content: PlantContents,
}

impl Page for PlantDetails {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let plant_header = Headline {
            attributes: vec![],
            size: HeaderSize::H1,
            content: {
                let plant_name_str = self.plant_name.clone();
                let plant_species_str = self.plant_species.clone().unwrap_or("".to_owned());
                Rc::new(format!("{plant_name_str} {plant_species_str}").into())
            },
        }
        .into();
        let body_content = vec![
            self.header.render(date_format),
            plant_header,
            self.plant_content.render(date_format),
            self.footer.render(date_format),
        ]
        .into();
        let body = Body {
            content: Rc::new(body_content),
        };
        HtmlDocument {
            head: Head::from(&self.get_head()),
            body,
        }
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "../css/main.css".to_owned(),
            "../css/header.css".to_owned(),
            "../css/footer.css".to_owned(),
        ];
        HtmlHead {
            title: self.plant_name.clone(),
            styles,
        }
    }
}
impl TryFrom<(&Plant, i32)> for PlantDetails {
    type Error = Error;
    fn try_from((plant, num_plants): (&Plant, i32)) -> Result<PlantDetails, Error> {
        let plant_species = plant.species.clone().map(|sp| sp.name.clone());
        let img_base = "../img/plants";
        let plant_content = PlantContents::try_from((plant, img_base))?;
        Ok(PlantDetails {
            header: Header::from(true),
            plant_name: plant.name.clone(),
            plant_species,
            plant_content,
            footer: Footer::from(num_plants),
        })
    }
}
