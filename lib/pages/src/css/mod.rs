mod classes;
mod footer;
mod gallery;
mod hall_of_fame;
mod header;
mod plant_details;
mod plant_list;
mod plant_search;
mod root;
mod species_details;
mod tags;
mod upcoming_tasks;

use crate::page::{CssComponent, PageComponent};
use html::elements::{HtmlElement, Style};

use classes::Classes;
use footer::Footer;
use gallery::Gallery;
use hall_of_fame::HallOfFame;
use header::Header;
use plant_details::PlantDetails;
use plant_list::PlantList;
use plant_search::PlantSearch;
use root::Root;
use species_details::SpeciesDetails;
use tags::Tags;
use upcoming_tasks::UpcomingTasks;

#[derive(Clone)]
pub enum PageCss {
    Activities,
    Gallery,
    Graveyard,
    Index,
    PlantDetails,
    PlantOverview,
    SpeciesDetails,
    SpeciesOverview,
}

impl PageComponent for PageCss {
    fn render(&self, _: &str) -> HtmlElement {
        match self {
            PageCss::Activities => Style {
                styles: vec![
                    Classes {}.render(),
                    Tags {}.render(),
                    Root {}.render(),
                    Header {}.render(),
                    Footer {}.render(),
                ],
            }
            .into(),
            PageCss::Gallery => Style {
                styles: vec![
                    Classes {}.render(),
                    Tags {}.render(),
                    Root {}.render(),
                    Header {}.render(),
                    Footer {}.render(),
                    Gallery {}.render(),
                ],
            }
            .into(),
            PageCss::Graveyard => Style {
                styles: vec![
                    Classes {}.render(),
                    Tags {}.render(),
                    Root {}.render(),
                    Header {}.render(),
                    Footer {}.render(),
                ],
            }
            .into(),
            PageCss::Index => Style {
                styles: vec![
                    Classes {}.render(),
                    Tags {}.render(),
                    Root {}.render(),
                    Header {}.render(),
                    Footer {}.render(),
                    HallOfFame {}.render(),
                    UpcomingTasks {}.render(),
                ],
            }
            .into(),
            PageCss::PlantDetails => Style {
                styles: vec![
                    Classes {}.render(),
                    Tags {}.render(),
                    Root {}.render(),
                    Header {}.render(),
                    Footer {}.render(),
                    Gallery {}.render(),
                    PlantDetails {}.render(),
                ],
            }
            .into(),
            PageCss::PlantOverview => Style {
                styles: vec![
                    Classes {}.render(),
                    Tags {}.render(),
                    Root {}.render(),
                    Header {}.render(),
                    Footer {}.render(),
                    PlantSearch {}.render(),
                    PlantList {}.render(),
                ],
            }
            .into(),
            PageCss::SpeciesDetails => Style {
                styles: vec![
                    Classes {}.render(),
                    Tags {}.render(),
                    Root {}.render(),
                    Header {}.render(),
                    Footer {}.render(),
                    Gallery {}.render(),
                    SpeciesDetails {}.render(),
                ],
            }
            .into(),
            PageCss::SpeciesOverview => Style {
                styles: vec![
                    Classes {}.render(),
                    Tags {}.render(),
                    Root {}.render(),
                    Header {}.render(),
                    Footer {}.render(),
                    PlantList {}.render(),
                ],
            }
            .into(),
        }
    }
}
