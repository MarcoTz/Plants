mod footer;
mod gallery;
mod hall_of_fame;
mod header;
mod main;
mod species_details;
mod upcoming_tasks;

use crate::page::CssComponent;
use html::css::CssDocument;

use footer::Footer;
use gallery::Gallery;
use hall_of_fame::HallOfFame;
use header::Header;
use main::Main;
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
        }
    }
}
