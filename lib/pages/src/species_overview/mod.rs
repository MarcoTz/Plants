pub mod species_list;
use super::{components::page_component::PageComponent, page::Page, shared::html_head::HtmlHead};
use species_list::SpeciesList;

use html::html_element::HtmlElement;
use plants::{plant::Plant, species::Species};

pub struct SpeciesOverview {
    pub species_list: SpeciesList,
}

impl Page for SpeciesOverview {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        self.species_list.render(date_format)
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "css/main.css".to_owned(),
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
            "css/plant_list.css".to_owned(),
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
        }
    }
}
