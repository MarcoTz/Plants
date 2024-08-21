pub mod species_list;
use super::{
    css::PageCss,
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
    fn get_title(&self) -> String {
        "Species".to_owned()
    }

    fn get_content(&self, date_format: &str) -> HtmlElement {
        self.species_list.render(date_format)
    }

    fn get_head(&self, date_format: &str) -> HtmlHead {
        let scripts = vec!["js/main.js".to_owned()];
        HtmlHead {
            title: self.get_title(),
            styles: PageCss::SpeciesOverview,
            scripts,
            date_format: date_format.to_owned(),
        }
    }
}

impl From<(&[Species], &[Plant])> for SpeciesOverview {
    fn from((species, plants): (&[Species], &[Plant])) -> SpeciesOverview {
        log::info!("Loading species overview");
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
