use super::errors::Error;
use database::database_manager::DatabaseManager;
use html::render::Render;
use pages::{
    components::{
        autowatered::AutoWatered, footer::Footer, hall_of_fame::HallOfFame, header::Header,
        next_activity::NextActivity,
    },
    index::Index,
    page::Page,
};

pub struct PageURLs {
    index_url: String,
    plant_overview_url: String,
    species_overview_url: String,
    gallery_url: String,
    activities_url: String,
    graveyard_url: String,
}
impl PageURLs {
    pub fn get_default() -> PageURLs {
        PageURLs {
            index_url: "index.html".to_owned(),
            plant_overview_url: "plant_overview.html".to_owned(),
            species_overview_url: "species_overview.html".to_owned(),
            gallery_url: "gallery.html".to_owned(),
            activities_url: "activities.html".to_owned(),
            graveyard_url: "graveyard.html".to_owned(),
        }
    }
}

pub struct Renderer<T: DatabaseManager> {
    pub database_manager: T,
    pub urls: PageURLs,
    pub date_format: String,
}

impl<T: DatabaseManager> Renderer<T> {
    fn get_header(&self, relative_up: bool) -> Header {
        let prefix = if relative_up {
            "../".to_owned()
        } else {
            "./".to_owned()
        };
        Header::from((
            prefix.clone() + &self.urls.index_url,
            prefix.clone() + &self.urls.plant_overview_url,
            prefix.clone() + &self.urls.species_overview_url,
            prefix.clone() + &self.urls.gallery_url,
            prefix.clone() + &self.urls.activities_url,
            prefix.clone() + &self.urls.graveyard_url,
        ))
    }

    pub fn render_index(&self) -> Result<String, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let hall_of_fame = HallOfFame::try_from(plants.as_slice())?;
        Ok(Index {
            header: self.get_header(false),
            next_activities: NextActivity::from(plants.as_slice()),
            autowatered: AutoWatered::from(plants.as_slice()),
            hall_of_fame,
            footer: Footer::from(plants.len() as i32),
        }
        .render(&self.date_format)
        .render())
    }
}
