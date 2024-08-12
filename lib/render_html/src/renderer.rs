use super::errors::Error;
use database::database_manager::DatabaseManager;
use html::render::Render;
use pages::{
    activities::Activities, gallery::Gallery, graveyard::Graveyard, index::Index, page::Page,
    plant_details::PlantDetails, plant_overview::PlantOverview, species_details::SpeciesDetails,
    species_overview::SpeciesOverview,
};

pub struct NamedPage {
    pub page_name: String,
    pub page_html: String,
}
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
        let plants = self.database_manager.get_all_plants()?;
        let index = Index::try_from(plants.as_slice())?;
        Ok(index
            .render(&self.date_format, false, plants.len() as i32)
            .render())
    }

    pub fn render_plant_overview(&mut self) -> Result<String, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let plant_overview = PlantOverview::try_from(plants.as_slice())?;
        Ok(plant_overview
            .render(&self.date_format, false, plants.len() as i32)
            .render())
    }

    pub fn render_species_overview(&mut self) -> Result<String, Error> {
        let species = self.database_manager.get_all_species()?;
        let plants = self.database_manager.get_all_plants()?;
        let species_overview = SpeciesOverview::from((species.as_slice(), plants.as_slice()));
        Ok(species_overview
            .render(&self.date_format, false, plants.len() as i32)
            .render())
    }

    pub fn render_gallery(&mut self) -> Result<String, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let gallery = Gallery::from(plants.as_slice());
        Ok(gallery
            .render(&self.date_format, false, plants.len() as i32)
            .render())
    }

    pub fn render_activities(&mut self) -> Result<String, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let activities = Activities::from(plants.as_slice());
        Ok(activities
            .render(&self.date_format, false, plants.len() as i32)
            .render())
    }

    pub fn render_graveyard(&mut self) -> Result<String, Error> {
        let graveyard_plants = self.database_manager.get_graveyard()?;
        let num_plants = self.database_manager.get_num_plants()?;
        let graveyard = Graveyard::from(graveyard_plants.as_slice());
        Ok(graveyard
            .render(&self.date_format, false, num_plants)
            .render())
    }

    pub fn render_all_plants(&mut self) -> Result<Vec<NamedPage>, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let num_plants = plants.len() as i32;
        let mut plant_htmls = vec![];
        for plant in plants.iter() {
            let plant_details = PlantDetails::try_from(plant)?;
            let page_html = plant_details
                .render(&self.date_format, true, num_plants)
                .render();
            let page_name = plant.get_url("plants/");
            plant_htmls.push(NamedPage {
                page_name,
                page_html,
            })
        }
        Ok(plant_htmls)
    }

    pub fn render_all_species(&mut self) -> Result<Vec<NamedPage>, Error> {
        let mut species_htmls = vec![];
        let all_species = self.database_manager.get_all_species()?;
        let all_plants = self.database_manager.get_all_plants()?;

        for species in all_species.iter() {
            let species_details = SpeciesDetails::from((species, all_plants.as_slice()));
            let species_html = species_details
                .render(&self.date_format, true, all_plants.len() as i32)
                .render();
            species_htmls.push(NamedPage {
                page_name: species.get_url("species/"),
                page_html: species_html,
            })
        }

        Ok(species_htmls)
    }

    pub fn render_all(&mut self) -> Result<PagesHtml, Error> {
        let index_html = self.render_index()?;
        let plants_overview_html = self.render_plant_overview()?;
        let species_overview_html = self.render_species_overview()?;
        let gallery_html = self.render_gallery()?;
        let activities_html = self.render_activities()?;
        let graveyard_html = self.render_graveyard()?;
        let plant_htmls = self.render_all_plants()?;
        let species_htmls = self.render_all_species()?;

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
