pub mod species_list;
use super::{
    components::page_component::PageComponent,
    page::Page,
    shared::{footer::Footer, header::Header, html_head::HtmlHead},
};
use species_list::SpeciesList;

use html::{body::Body, head::Head, html_document::HtmlDocument};
use plants::{plant::Plant, species::Species};
use std::rc::Rc;

pub struct SpeciesOverview {
    pub header: Header,
    pub footer: Footer,
    pub species_list: SpeciesList,
}

impl Page for SpeciesOverview {
    fn render(&self, date_format: &str) -> HtmlDocument {
        let body_contents = vec![
            self.header.render(date_format),
            self.species_list.render(date_format),
            self.footer.render(date_format),
        ];
        let body = Body {
            content: Rc::new(body_contents.into()),
        };
        HtmlDocument {
            head: Head::from(&self.get_head()),
            body,
        }
    }
    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "css/main.css".to_owned(),
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
        ];
        HtmlHead {
            title: "Species".to_owned(),
            styles,
        }
    }
}

impl From<(&[Species], &[Plant])> for SpeciesOverview {
    fn from((species, plants): (&[Species], &[Plant])) -> SpeciesOverview {
        let species_plants: Vec<(&Species, Vec<Plant>)> = species
            .iter()
            .map(|sp| (sp, sp.get_plants(plants)))
            .collect();
        SpeciesOverview {
            species_list: SpeciesList::from(
                species_plants
                    .into_iter()
                    .map(|(sp, plants)| (sp, plants.first().cloned()))
                    .collect::<Vec<(&Species, Option<Plant>)>>()
                    .as_slice(),
            ),
            header: Header::from(false),
            footer: Footer::from(plants.len() as i32),
        }
    }
}
