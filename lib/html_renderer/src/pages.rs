use std::path;
use std::slice::Iter;

pub enum PageType {
    Full,
    Component,
}
pub enum Page {
    Index,
    Activities,
    Gallery,
    Graveyard,
    Header,
    Footer,
    ImageView,
    PlantDetails,
    PlantOverview,
    SpeciesDetails,
    SpeciesOverview,
}

impl Page {
    pub fn iterator() -> Iter<'static, Page> {
        static PAGES: [Page; 11] = [
            Page::Index,
            Page::Activities,
            Page::Gallery,
            Page::Graveyard,
            Page::Header,
            Page::Footer,
            Page::ImageView,
            Page::PlantDetails,
            Page::PlantOverview,
            Page::SpeciesDetails,
            Page::SpeciesOverview,
        ];
        PAGES.iter()
    }

    fn get_page_type(&self) -> PageType {
        match self {
            Page::Index => PageType::Full,
            Page::Activities => PageType::Component,
            Page::Gallery => PageType::Full,
            Page::Graveyard => PageType::Full,
            Page::Header => PageType::Component,
            Page::Footer => PageType::Component,
            Page::ImageView => PageType::Component,
            Page::PlantDetails => PageType::Full,
            Page::PlantOverview => PageType::Full,
            Page::SpeciesDetails => PageType::Full,
            Page::SpeciesOverview => PageType::Full,
        }
    }

    pub fn get_page_name(&self) -> String {
        match self {
            Page::Index => "index".to_owned(),
            Page::Activities => "activities".to_owned(),
            Page::Gallery => "gallery".to_owned(),
            Page::Graveyard => "graveyard".to_owned(),
            Page::Header => "header".to_owned(),
            Page::Footer => "footer".to_owned(),
            Page::ImageView => "image_vew".to_owned(),
            Page::PlantDetails => "plant_details".to_owned(),
            Page::PlantOverview => "plant_overview".to_owned(),
            Page::SpeciesDetails => "species_details".to_owned(),
            Page::SpeciesOverview => "species_overview".to_owned(),
        }
    }

    pub fn get_page_path(&self, template_dir: &str) -> path::PathBuf {
        let mut file_name = self.get_page_name();
        file_name.push_str(".html");
        path::Path::new(template_dir).join(&file_name)
    }
}
