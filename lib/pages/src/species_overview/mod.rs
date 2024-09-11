pub mod species_list;
use super::{
    css::PageCss,
    page::{Page, PageComponent},
    shared::html_head::HtmlHead,
};
use species_list::SpeciesList;

use html::elements::HtmlElement;
use plants::{plant::Plant, species::Species};

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod species_overiew_tests {
    use super::{HtmlHead, Page, PageComponent, PageCss, SpeciesList, SpeciesOverview};
    use crate::test_common::{example_plant1, example_species, DATE_FORMAT};

    fn example_overview() -> SpeciesOverview {
        SpeciesOverview {
            species_list: SpeciesList::from(
                vec![(&example_species(), Some(example_plant1()))].as_slice(),
            ),
        }
    }

    #[test]
    fn overview_title() {
        let result = example_overview().get_title();
        let expected = "Species";
        assert_eq!(result, expected)
    }

    #[test]
    fn overview_content() {
        let result = example_overview().get_content(DATE_FORMAT);
        let expected =
            SpeciesList::from(vec![(&example_species(), Some(example_plant1()))].as_slice())
                .render(DATE_FORMAT);
        assert_eq!(result, expected)
    }

    #[test]
    fn overview_head() {
        let result = example_overview().get_head(DATE_FORMAT);
        let expected = HtmlHead {
            title: "Species".to_owned(),
            styles: PageCss::SpeciesOverview,
            scripts: vec!["js/main.js".to_owned()],
            date_format: DATE_FORMAT.to_owned(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn overview_into() {
        let result = SpeciesOverview::from((
            vec![example_species()].as_slice(),
            vec![example_plant1()].as_slice(),
        ));
        let expected = example_overview();
        assert_eq!(result, expected)
    }
}
