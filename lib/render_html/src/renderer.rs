use super::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use html::render::Render;
use pages::{
    components::{
        autowatered::AutoWatered, footer::Footer, hall_of_fame::HallOfFame, header::Header,
        html_head::HtmlHead, next_activity::NextActivity,
    },
    index::Index,
    page::Page,
};

pub struct PageURLs {
    pub index_url: String,
    pub plant_overview_url: String,
    pub species_overview_url: String,
    pub gallery_url: String,
    pub activities_url: String,
    pub graveyard_url: String,
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
    fn get_prefix(relative_up: bool) -> String {
        if relative_up {
            "../".to_owned()
        } else {
            "./".to_owned()
        }
    }

    fn get_header(&self, relative_up: bool) -> Header {
        let prefix = Renderer::<T>::get_prefix(relative_up);
        Header {
            dashboard_url: prefix.clone() + &self.urls.index_url,
            plants_url: prefix.clone() + &self.urls.plant_overview_url,
            species_url: prefix.clone() + &self.urls.species_overview_url,
            gallery_url: prefix.clone() + &self.urls.gallery_url,
            activities_url: prefix.clone() + &self.urls.activities_url,
            graveyard_url: prefix.clone() + &self.urls.graveyard_url,
        }
    }

    fn get_head(&self, title: &str, relative_up: bool) -> HtmlHead {
        let prefix = Renderer::<T>::get_prefix(relative_up);
        HtmlHead {
            title: title.to_owned(),
            styles: vec![prefix.clone() + "css/main.css", prefix + "css/index.css"],
        }
    }

    pub fn render_index(&self) -> Result<String, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let hall_of_fame = HallOfFame::try_from(plants.as_slice())?;
        Ok(Index {
            head: self.get_head("Dashboard", false),
            header: self.get_header(false),
            next_activities: NextActivity::from(plants.as_slice()),
            autowatered: AutoWatered::from(plants.as_slice()),
            hall_of_fame,
            footer: Footer {
                num_plants: plants.len() as i32,
                last_build: Local::now().date_naive(),
            },
        }
        .render(&self.date_format)
        .render())
    }
}