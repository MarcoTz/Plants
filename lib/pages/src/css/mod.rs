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
use html::elements::HtmlElement;

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
            PageCss::Activities => vec![
                Classes {}.render().into(),
                Tags {}.render().into(),
                Root {}.render().into(),
                Header {}.render().into(),
                Footer {}.render().into(),
            ]
            .into(),

            PageCss::Gallery => vec![
                Classes {}.render().into(),
                Tags {}.render().into(),
                Root {}.render().into(),
                Header {}.render().into(),
                Footer {}.render().into(),
                Gallery {}.render().into(),
            ]
            .into(),
            PageCss::Graveyard => vec![
                Classes {}.render().into(),
                Tags {}.render().into(),
                Root {}.render().into(),
                Header {}.render().into(),
                Footer {}.render().into(),
            ]
            .into(),
            PageCss::Index => vec![
                Classes {}.render().into(),
                Tags {}.render().into(),
                Root {}.render().into(),
                Header {}.render().into(),
                Footer {}.render().into(),
                HallOfFame {}.render().into(),
                UpcomingTasks {}.render().into(),
            ]
            .into(),

            PageCss::PlantDetails => vec![
                Classes {}.render().into(),
                Tags {}.render().into(),
                Root {}.render().into(),
                Header {}.render().into(),
                Footer {}.render().into(),
                Gallery {}.render().into(),
                PlantDetails {}.render().into(),
            ]
            .into(),
            PageCss::PlantOverview => vec![
                Classes {}.render().into(),
                Tags {}.render().into(),
                Root {}.render().into(),
                Header {}.render().into(),
                Footer {}.render().into(),
                PlantSearch {}.render().into(),
                PlantList {}.render().into(),
            ]
            .into(),
            PageCss::SpeciesDetails => vec![
                Classes {}.render().into(),
                Tags {}.render().into(),
                Root {}.render().into(),
                Header {}.render().into(),
                Footer {}.render().into(),
                Gallery {}.render().into(),
                SpeciesDetails {}.render().into(),
            ]
            .into(),
            PageCss::SpeciesOverview => vec![
                Classes {}.render().into(),
                Tags {}.render().into(),
                Root {}.render().into(),
                Header {}.render().into(),
                Footer {}.render().into(),
                PlantList {}.render().into(),
            ]
            .into(),
        }
    }
}
