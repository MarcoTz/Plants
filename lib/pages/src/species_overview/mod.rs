pub mod species_list;
use super::{
    css::DefinedDocument,
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use species_list::SpeciesList;

use html::elements::HtmlElement;
use plants::{plant::Plant, species::Species};

pub struct SpeciesOverview {
    pub species_list: SpeciesList,
}

impl Page for SpeciesOverview {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        self.species_list.render(date_format)
    }

    fn get_head(&self) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: "Species".to_owned(),
            styles_extern: vec![],
            styles: vec![
                DefinedDocument::Main,
                DefinedDocument::Header,
                DefinedDocument::Footer,
                DefinedDocument::PlantList,
            ],
            scripts,
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
