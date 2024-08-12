use super::errors::Error;
use chrono::Local;
use database::database_manager::DatabaseManager;
use html::render::Render;
use pages::{
    activities::Activities,
    components::{
        graveyard_table::GraveyardTable, html_head::HtmlHead,
        plant_activity_table::PlantActivityTable, plant_contents::PlantContents,
        species_gallery::SpeciesGallery, species_info::SpeciesInfo, species_list::SpeciesList,
    },
    gallery::Gallery,
    graveyard::Graveyard,
    index::Index,
    index::{autowatered::AutoWatered, hall_of_fame::HallOfFame, upcoming_tasks::UpcomingTasks},
    page::Page,
    plant_details::PlantDetails,
    plant_overview::plant_search::PlantSearch,
    plant_overview::PlantOverview,
    shared::{footer::Footer, header::Header, plant_list::PlantList},
    species_details::SpeciesDetails,
    species_overview::SpeciesOverview,
};
use plants::{plant::Plant, species::Species};

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

    fn get_head(&self, title: &str, relative_up: bool, additional_styles: Vec<&str>) -> HtmlHead {
        let prefix = Renderer::<T>::get_prefix(relative_up);
        let mut styles = vec![
            prefix.clone() + "css/main.css",
            prefix.clone() + "css/header.css",
            prefix.clone() + "css/footer.css",
        ];
        for additional_style in additional_styles.iter() {
            styles.push(prefix.clone() + additional_style);
        }
        HtmlHead {
            title: title.to_owned(),
            styles,
        }
    }

    pub fn render_index(&mut self) -> Result<String, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let hall_of_fame = HallOfFame::try_from(plants.as_slice())?;
        Ok(Index {
            head: self.get_head(
                "Dashboard",
                false,
                vec![
                    "css/index.css",
                    "css/upcoming_tasks.css",
                    "css/hall_of_fame.css",
                ],
            ),
            header: self.get_header(false),
            next_activities: UpcomingTasks::from(plants.as_slice()),
            autowatered: AutoWatered::from(plants.as_slice()),
            hall_of_fame,
            footer: self.get_footer(plants.len() as i32),
        }
        .render(&self.date_format)
        .render())
    }

    pub fn render_plant_overview(&mut self) -> Result<String, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let plant_list = PlantList::try_from(plants.as_slice())?;
        Ok(PlantOverview {
            head: self.get_head(
                "All Plants",
                false,
                vec![
                    "css/plant_overview.css",
                    "css/plant_search.css",
                    "css/plant_list.css",
                ],
            ),
            header: self.get_header(false),
            search: PlantSearch {},
            plant_list,
            footer: self.get_footer(plants.len() as i32),
        }
        .render(&self.date_format)
        .render())
    }

    pub fn render_species_overview(&mut self) -> Result<String, Error> {
        let num_plants = self.database_manager.get_num_plants()?;
        let species = self.database_manager.get_all_species()?;
        let species_w_plants: Vec<(Species, Option<Plant>)> = species
            .iter()
            .map(|sp| {
                let species_plants = self.database_manager.get_plants_species(&sp.name).ok();
                match species_plants {
                    None => (sp.clone(), None),
                    Some(plants) => match plants.first() {
                        None => (sp.clone(), None),
                        Some(pl) => (sp.clone(), Some(pl.clone())),
                    },
                }
            })
            .collect();
        Ok(SpeciesOverview {
            head: self.get_head("All Species", false, vec!["css/species_overview.css"]),
            species_list: SpeciesList::from(species_w_plants.as_slice()),
            header: self.get_header(false),
            footer: self.get_footer(num_plants),
        }
        .render(&self.date_format)
        .render())
    }

    pub fn render_gallery(&mut self) -> Result<String, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let img_base = "img/plants";
        let plant_galleries = plants.iter().map(|x| (x, img_base).into()).collect();
        let num_plants = plants.len() as i32;
        Ok(Gallery {
            head: self.get_head("Gallery", false, vec!["css/gallery.css"]),
            header: self.get_header(false),
            plant_galleries,
            footer: self.get_footer(num_plants),
        }
        .render(&self.date_format)
        .render())
    }

    pub fn render_activities(&mut self) -> Result<String, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let num_plants = plants.len() as i32;
        Ok(Activities {
            head: self.get_head("Activities", false, vec![]),
            header: self.get_header(false),
            activity_table: PlantActivityTable::from((plants.as_slice(), true, true)),
            footer: self.get_footer(num_plants),
        }
        .render(&self.date_format)
        .render())
    }

    pub fn render_graveyard(&mut self) -> Result<String, Error> {
        let graveyard = self.database_manager.get_graveyard()?;
        let num_plants = self.database_manager.get_num_plants()?;
        Ok(Graveyard {
            head: self.get_head("Graveyard", false, vec![]),
            header: self.get_header(false),
            footer: self.get_footer(num_plants),
            graveyard_table: GraveyardTable::from(graveyard.as_slice()),
        }
        .render(&self.date_format)
        .render())
    }

    pub fn render_all_plants(&mut self) -> Result<Vec<NamedPage>, Error> {
        let plants = self.database_manager.get_all_plants()?;
        let num_plants = plants.len() as i32;
        let img_base = "../img/plants";
        let mut plant_htmls = vec![];
        for plant in plants.iter() {
            let plant_content = PlantContents::try_from((plant, img_base))?;
            let plant_species = plant.species.clone().map(|sp| sp.name.clone());
            let page_html = PlantDetails {
                head: self.get_head(
                    &plant.name,
                    true,
                    vec!["css/plant_details.css", "css/gallery.css"],
                ),
                header: self.get_header(true),
                plant_name: plant.name.clone(),
                plant_species,
                plant_content,
                footer: self.get_footer(num_plants),
            }
            .render(&self.date_format)
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
        let num_plants = self.database_manager.get_num_plants()?;
        let all_species = self.database_manager.get_all_species()?;
        let img_base = "img/plants";
        for species in all_species.iter() {
            let species_plants = self
                .database_manager
                .get_plants_species(species.name.as_str())?;
            let species_html = SpeciesDetails {
                head: self.get_head(
                    &species.name,
                    true,
                    vec!["css/species_details.css", "css/gallery.css"],
                ),
                header: self.get_header(true),
                species_name: species.name.clone(),
                species_info: SpeciesInfo::from((species, species_plants.as_slice())),
                species_gallery: SpeciesGallery::from((species_plants.as_slice(), img_base)),
                footer: self.get_footer(num_plants),
            }
            .render(&self.date_format)
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
