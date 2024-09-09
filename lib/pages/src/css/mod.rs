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

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod pagecss_tests {
    use super::{
        Classes, CssComponent, Footer, Gallery, HallOfFame, Header, PageComponent, PageCss,
        PlantDetails, PlantList, PlantSearch, Root, SpeciesDetails, Tags, UpcomingTasks,
    };
    use crate::test_common::DATE_FORMAT;

    #[test]
    fn render_activities() {
        let result = PageCss::Activities.render(DATE_FORMAT);
        let expected = vec![
            Classes {}.render().into(),
            Tags {}.render().into(),
            Root {}.render().into(),
            Header {}.render().into(),
            Footer {}.render().into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_gallery() {
        let result = PageCss::Gallery.render(DATE_FORMAT);
        let expected = vec![
            Classes {}.render().into(),
            Tags {}.render().into(),
            Root {}.render().into(),
            Header {}.render().into(),
            Footer {}.render().into(),
            Gallery {}.render().into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_index() {
        let result = PageCss::Index.render(DATE_FORMAT);
        let expected = vec![
            Classes {}.render().into(),
            Tags {}.render().into(),
            Root {}.render().into(),
            Header {}.render().into(),
            Footer {}.render().into(),
            HallOfFame {}.render().into(),
            UpcomingTasks {}.render().into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_plantdetails() {
        let result = PageCss::PlantDetails.render(DATE_FORMAT);
        let expected = vec![
            Classes {}.render().into(),
            Tags {}.render().into(),
            Root {}.render().into(),
            Header {}.render().into(),
            Footer {}.render().into(),
            Gallery {}.render().into(),
            PlantDetails {}.render().into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_plantoverview() {
        let result = PageCss::PlantOverview.render(DATE_FORMAT);
        let expected = vec![
            Classes {}.render().into(),
            Tags {}.render().into(),
            Root {}.render().into(),
            Header {}.render().into(),
            Footer {}.render().into(),
            PlantSearch {}.render().into(),
            PlantList {}.render().into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_speciesdetails() {
        let result = PageCss::SpeciesDetails.render(DATE_FORMAT);
        let expected = vec![
            Classes {}.render().into(),
            Tags {}.render().into(),
            Root {}.render().into(),
            Header {}.render().into(),
            Footer {}.render().into(),
            Gallery {}.render().into(),
            SpeciesDetails {}.render().into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_speciesoverview() {
        let result = PageCss::SpeciesOverview.render(DATE_FORMAT);
        let expected = vec![
            Classes {}.render().into(),
            Tags {}.render().into(),
            Root {}.render().into(),
            Header {}.render().into(),
            Footer {}.render().into(),
            PlantList {}.render().into(),
        ]
        .into();
        assert_eq!(result, expected)
    }
}
