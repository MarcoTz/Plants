use super::shared::html_head::HtmlHead;
use html::html_document::HtmlDocument;

pub trait Page {
    fn render(&self, date_format: &str) -> HtmlDocument;
    fn get_head(&self) -> HtmlHead;
}

pub enum PageURLs {
    IndexUrl,
    PlantsOverviewUrl,
    SpeciesOverviewUrl,
    GalleryUrl,
    ActivitiesUrl,
    GraveyardUrl,
}
impl PageURLs {
    pub fn get_url(self) -> String {
        match self {
            PageURLs::IndexUrl => "index.html".to_owned(),
            PageURLs::PlantsOverviewUrl => "plant_overview.html".to_owned(),
            PageURLs::SpeciesOverviewUrl => "species_overview.html".to_owned(),
            PageURLs::GalleryUrl => "gallery.html".to_owned(),
            PageURLs::ActivitiesUrl => "activities.html".to_owned(),
            PageURLs::GraveyardUrl => "graveyard.html".to_owned(),
        }
    }
}
