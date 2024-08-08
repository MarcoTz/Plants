use super::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use html::render::Render;
use pages::{
    components::{
        autowatered::AutoWatered, footer::Footer, graveyard_table::GraveyardTable,
        hall_of_fame::HallOfFame, header::Header, html_head::HtmlHead, next_activity::NextActivity,
    },
    graveyard::Graveyard,
    index::Index,
    page::Page,
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

    fn get_footer(&self, num_plants: i32) -> Footer {
        Footer {
            num_plants,
            last_build: Local::now().date_naive(),
        }
    }

    fn get_head(&self, title: &str, relative_up: bool) -> HtmlHead {
        let prefix = Renderer::<T>::get_prefix(relative_up);
        HtmlHead {
            title: title.to_owned(),
            styles: vec![prefix.clone() + "css/main.css", prefix + "css/index.css"],
        }
    }

    pub fn render_index(&mut self) -> Result<String, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let hall_of_fame = HallOfFame::try_from(plants.as_slice())?;
        Ok(Index {
            head: self.get_head("Dashboard", false),
            header: self.get_header(false),
            next_activities: NextActivity::from(plants.as_slice()),
            autowatered: AutoWatered::from(plants.as_slice()),
            hall_of_fame,
            footer: self.get_footer(plants.len() as i32),
        }
        .render(&self.date_format)
        .render())
    }

    pub fn render_plant_overview(&self) -> Result<String, Error> {
        Ok("".to_owned())
    }

    pub fn render_species_overview(&self) -> Result<String, Error> {
        Ok("".to_owned())
    }

    pub fn render_gallery(&self) -> Result<String, Error> {
        Ok("".to_owned())
    }

    pub fn render_activities(&self) -> Result<String, Error> {
        Ok("".to_owned())
    }

    pub fn render_graveyard(&mut self) -> Result<String, Error> {
        let graveyard = self.database_manager.get_graveyard()?;
        let num_plants = self.database_manager.get_num_plants()?;
        Ok(Graveyard {
            head: self.get_head("Graveyard", false),
            header: self.get_header(false),
            footer: self.get_footer(num_plants),
            graveyard_table: GraveyardTable::from(graveyard.as_slice()),
        }
        .render(&self.date_format)
        .render())
    }

    pub fn render_all_plants(&self) -> Result<Vec<NamedPage>, Error> {
        Ok(vec![])
    }

    pub fn render_all_species(&self) -> Result<Vec<NamedPage>, Error> {
        Ok(vec![])
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
