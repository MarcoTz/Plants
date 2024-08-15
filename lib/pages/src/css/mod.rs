mod footer;
mod gallery;
mod hall_of_fame;
mod header;
mod main;
mod plant_details;
mod plant_list;
mod plant_search;
mod species_details;
mod upcoming_tasks;

use crate::page::CssComponent;
use html::css::CssDocument;

use footer::Footer;
use gallery::Gallery;
use hall_of_fame::HallOfFame;
use header::Header;
use main::Main;
use plant_details::PlantDetails;
use plant_list::PlantList;
use plant_search::PlantSearch;
use species_details::SpeciesDetails;
use upcoming_tasks::UpcomingTasks;

#[derive(Clone)]
pub enum DefinedDocument {
    Main,
    Header,
    Footer,
    Gallery,
    HallOfFame,
    UpcomingTasks,
    SpeciesDetails,
    PlantDetails,
    PlantSearch,
    PlantList,
}

impl CssComponent for DefinedDocument {
    fn render(&self) -> CssDocument {
        match self {
            DefinedDocument::Main => Main {}.render(),
            DefinedDocument::Header => Header {}.render(),
            DefinedDocument::Footer => Footer {}.render(),
            DefinedDocument::Gallery => Gallery {}.render(),
            DefinedDocument::HallOfFame => HallOfFame {}.render(),
            DefinedDocument::UpcomingTasks => UpcomingTasks {}.render(),
            DefinedDocument::SpeciesDetails => SpeciesDetails {}.render(),
            DefinedDocument::PlantDetails => PlantDetails {}.render(),
            DefinedDocument::PlantSearch => PlantSearch {}.render(),
            DefinedDocument::PlantList => PlantList {}.render(),
        }
    }
}
