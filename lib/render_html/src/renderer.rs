use super::errors::Error;
use database::database_manager::DatabaseManager;
use html::render::Render;
use log;
use pages::{
    activities::Activities, gallery::Gallery, graveyard::Graveyard, index::Index, page::Page,
    plant_details::PlantDetails, plant_overview::PlantOverview, species_details::SpeciesDetails,
    species_overview::SpeciesOverview,
};

#[derive(Debug, PartialEq, Eq)]
pub struct NamedPage {
    pub page_name: String,
    pub page_html: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PagesHtml {
    pub index_html: String,
    pub plants_overview_html: String,
    pub species_overview_html: String,
    pub gallery_html: String,
    pub activities_html: String,
    pub graveyard_html: String,
    pub plant_htmls: Vec<NamedPage>,
    pub species_htmls: Vec<NamedPage>,
}

pub struct Renderer<T: DatabaseManager> {
    pub database_manager: T,
    pub date_format: String,
}

impl<T: DatabaseManager> Renderer<T> {
    pub fn render_index(&mut self) -> Result<String, Error> {
        log::info!("Bulding index");
        let plants = self.database_manager.get_all_plants()?;
        let index = Index::try_from(plants.as_slice())?;
        Ok(index
            .render(&self.date_format, false, plants.len() as i32)
            .render())
    }

    pub fn render_plant_overview(&mut self) -> Result<String, Error> {
        log::info!("Building plant overview");
        let plants = self.database_manager.get_all_plants()?;
        let plant_overview = PlantOverview::from(plants.as_slice());
        Ok(plant_overview
            .render(&self.date_format, false, plants.len() as i32)
            .render())
    }

    pub fn render_species_overview(&mut self) -> Result<String, Error> {
        log::info!("Building Species Overview");
        let species = self.database_manager.get_all_species()?;
        let plants = self.database_manager.get_all_plants()?;
        let species_overview = SpeciesOverview::from((species.as_slice(), plants.as_slice()));
        Ok(species_overview
            .render(&self.date_format, false, plants.len() as i32)
            .render())
    }

    pub fn render_gallery(&mut self) -> Result<String, Error> {
        log::info!("Building Gallery");
        let plants = self.database_manager.get_all_plants()?;
        let gallery = Gallery::from(plants.as_slice());
        Ok(gallery
            .render(&self.date_format, false, plants.len() as i32)
            .render())
    }

    pub fn render_activities(&mut self) -> Result<String, Error> {
        log::info!("Building Activities");
        let plants = self.database_manager.get_all_plants()?;
        let activities = Activities::from(plants.as_slice());
        Ok(activities
            .render(&self.date_format, false, plants.len() as i32)
            .render())
    }

    pub fn render_graveyard(&mut self) -> Result<String, Error> {
        log::info!("Building Graveyard");
        let graveyard_plants = self.database_manager.get_graveyard()?;
        let num_plants = self.database_manager.get_num_plants()?;
        let graveyard = Graveyard::from(graveyard_plants.as_slice());
        Ok(graveyard
            .render(&self.date_format, false, num_plants)
            .render())
    }

    pub fn render_all_plants(&mut self) -> Result<Vec<NamedPage>, Error> {
        log::info!("Rendering Plant Details");
        let plants = self.database_manager.get_all_plants()?;
        let num_plants = plants.len() as i32;
        let mut plant_htmls = vec![];
        for plant in plants.iter() {
            log::info!("Rendering Details Page for plant {}", plant.info.name);
            let plant_details = PlantDetails::try_from(plant)?;
            let page_html = plant_details
                .render(&self.date_format, true, num_plants)
                .render();
            let page_name = plant.get_url("plants");
            plant_htmls.push(NamedPage {
                page_name,
                page_html,
            })
        }
        Ok(plant_htmls)
    }

    pub fn render_all_species(&mut self) -> Result<Vec<NamedPage>, Error> {
        log::info!("Rendering Species Details");
        let mut species_htmls = vec![];
        let all_species = self.database_manager.get_all_species()?;
        let all_plants = self.database_manager.get_all_plants()?;

        for species in all_species.iter() {
            log::info!("Rendering Details Page for spieces {}", species.name);
            let species_details = SpeciesDetails::from((species, all_plants.as_slice()));
            let species_html = species_details
                .render(&self.date_format, true, all_plants.len() as i32)
                .render();
            species_htmls.push(NamedPage {
                page_name: species.get_url("species"),
                page_html: species_html,
            })
        }

        Ok(species_htmls)
    }

    pub fn render_all(&mut self) -> Result<PagesHtml, Error> {
        log::info!("Rendering all pages");
        let index_html = self.render_index()?;
        let plants_overview_html = self.render_plant_overview()?;
        let species_overview_html = self.render_species_overview()?;
        let gallery_html = self.render_gallery()?;
        let activities_html = self.render_activities()?;
        let graveyard_html = self.render_graveyard()?;
        let plant_htmls = self.render_all_plants()?;
        let species_htmls = self.render_all_species()?;
        log::info!("Rendered all pages");

        Ok(PagesHtml {
            index_html,
            plants_overview_html,
            species_overview_html,
            gallery_html,
            activities_html,
            graveyard_html,
            plant_htmls,
            species_htmls,
        })
    }
}

#[cfg(test)]
mod renderer_tests {
    use super::{NamedPage, PagesHtml};
    use crate::test_common::{
        example_graveyard, example_plant, example_plant2, example_renderer, example_species,
        DATE_FORMAT,
    };
    use html::render::Render;
    use pages::{
        activities::Activities, gallery::Gallery, graveyard::Graveyard, index::Index, page::Page,
        plant_details::PlantDetails, plant_overview::PlantOverview,
        species_details::SpeciesDetails, species_overview::SpeciesOverview,
    };

    #[test]
    fn index() {
        let result = example_renderer().render_index().unwrap();
        let expected = Index::try_from(vec![example_plant(), example_plant2()].as_slice())
            .unwrap()
            .render(DATE_FORMAT, false, 2)
            .render();
        assert_eq!(result, expected)
    }

    #[test]
    fn plant_overview() {
        let result = example_renderer().render_plant_overview().unwrap();
        let expected = PlantOverview::from(vec![example_plant(), example_plant2()].as_slice())
            .render(DATE_FORMAT, false, 2)
            .render();
        assert_eq!(result, expected)
    }

    #[test]
    fn species_overview() {
        let result = example_renderer().render_species_overview().unwrap();
        let expected = SpeciesOverview::from((
            vec![example_species()].as_slice(),
            vec![example_plant(), example_plant2()].as_slice(),
        ))
        .render(DATE_FORMAT, false, 2)
        .render();
        assert_eq!(result, expected)
    }

    #[test]
    fn gallery() {
        let result = example_renderer().render_gallery().unwrap();
        let expected = Gallery::from(vec![example_plant(), example_plant2()].as_slice())
            .render(DATE_FORMAT, false, 2)
            .render();
        assert_eq!(result, expected)
    }

    #[test]
    fn activities() {
        let result = example_renderer().render_activities().unwrap();
        let expected = Activities::from(vec![example_plant(), example_plant2()].as_slice())
            .render(DATE_FORMAT, false, 2)
            .render();
        assert_eq!(result, expected)
    }

    #[test]
    fn graveyard() {
        let result = example_renderer().render_graveyard().unwrap();
        let expected = Graveyard::from(vec![example_graveyard()].as_slice())
            .render(DATE_FORMAT, false, 2)
            .render();
        assert_eq!(result, expected)
    }

    #[test]
    fn all_plants() {
        let result = example_renderer().render_all_plants().unwrap();
        let expected = vec![
            NamedPage {
                page_name: example_plant().get_url("plants"),
                page_html: PlantDetails::try_from(&example_plant())
                    .unwrap()
                    .render(DATE_FORMAT, true, 2)
                    .render(),
            },
            NamedPage {
                page_name: example_plant2().get_url("plants"),
                page_html: PlantDetails::try_from(&example_plant2())
                    .unwrap()
                    .render(DATE_FORMAT, true, 2)
                    .render(),
            },
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn all_species() {
        let result = example_renderer().render_all_species().unwrap();
        let expected = vec![NamedPage {
            page_name: example_species().get_url("species"),
            page_html: SpeciesDetails::from((
                &example_species(),
                vec![example_plant(), example_plant2()].as_slice(),
            ))
            .render(DATE_FORMAT, true, 2)
            .render(),
        }];
        assert_eq!(result, expected)
    }

    #[test]
    fn all() {
        let result = example_renderer().render_all().unwrap();
        let expected = PagesHtml {
            index_html: example_renderer().render_index().unwrap(),
            plants_overview_html: example_renderer().render_plant_overview().unwrap(),
            species_overview_html: example_renderer().render_species_overview().unwrap(),
            gallery_html: example_renderer().render_gallery().unwrap(),
            activities_html: example_renderer().render_activities().unwrap(),
            graveyard_html: example_renderer().render_graveyard().unwrap(),
            plant_htmls: example_renderer().render_all_plants().unwrap(),
            species_htmls: example_renderer().render_all_species().unwrap(),
        };
        assert_eq!(result, expected)
    }
}
