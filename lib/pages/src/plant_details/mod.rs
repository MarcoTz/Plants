pub mod activities;
pub mod activity_table;
pub mod graphs;
pub mod growth_table;
pub mod status;

use super::{
    errors::Error,
    page::{Page, PageComponent},
    shared::{html_head::HtmlHead, plant_gallery::PlantGallery, species_link::SpeciesLink},
};
use activities::PlantActivities;
use graphs::PlantGraphs;
use html::{
    attribute::Attribute,
    body::Body,
    div::Div,
    head::Head,
    headline::{HeaderSize, Headline},
    html_document::HtmlDocument,
    html_element::HtmlElement,
};
use plants::plant::Plant;
use status::Status;
use std::rc::Rc;

pub struct PlantDetails {
    pub name: String,
    pub species_link: Option<SpeciesLink>,
    pub gallery: PlantGallery,
    pub status: Status,
    pub growth: PlantGraphs,
    pub activities: PlantActivities,
}

impl Page for PlantDetails {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        let species_link = self
            .species_link
            .clone()
            .map(|lk| {
                vec![
                    "(".to_owned().into(),
                    lk.render(date_format),
                    ")".to_owned().into(),
                ]
                .into()
            })
            .unwrap_or("".to_owned().into());
        let header = Headline {
            attributes: vec![],
            size: HeaderSize::H1,
            content: Rc::new(vec![(self.name.clone() + " ").into(), species_link].into()),
        }
        .into();

        vec![
            header,
            Div {
                attributes: vec![Attribute::Id("plant_content".to_owned())],
                content: Rc::new(
                    vec![
                        self.gallery.render(date_format),
                        Div {
                            attributes: vec![
                                Attribute::Id("plant_info".to_owned()),
                                Attribute::Class(vec!["flex_container".to_owned()]),
                            ],
                            content: Rc::new(
                                vec![
                                    self.status.render(date_format),
                                    self.growth.render(date_format),
                                ]
                                .into(),
                            ),
                        }
                        .into(),
                        self.activities.render(date_format),
                    ]
                    .into(),
                ),
            }
            .into(),
        ]
        .into()
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "../css/main.css".to_owned(),
            "../css/header.css".to_owned(),
            "../css/footer.css".to_owned(),
            "../css/plant_details.css".to_owned(),
            "../css/gallery.css".to_owned(),
        ];
        let scripts = vec![
            "../js/graphs.js".to_owned(),
            "https://cdnjs.cloudflare.com/ajax/libs/Chart.js/2.9.4/Chart.js".to_owned(),
            "../js/main.js".to_owned(),
        ];
        HtmlHead {
            title: self.name.clone(),
            styles,
            scripts,
        }
    }

    fn render(&self, date_format: &str, relative_up: bool, num_plants: i32) -> HtmlDocument {
        let body_contents: HtmlElement = vec![
            self.get_header(relative_up).render(date_format),
            self.get_content(date_format),
            self.get_footer(num_plants).render(date_format),
        ]
        .into();
        HtmlDocument {
            head: Head::from(&self.get_head()),
            body: Body {
                attributes: vec![Attribute::OnLoad(
                    "create_graphs();setup_img_events()".to_owned(),
                )],
                content: Rc::new(body_contents),
            },
        }
    }
}
impl TryFrom<&Plant> for PlantDetails {
    type Error = Error;
    fn try_from(plant: &Plant) -> Result<PlantDetails, Error> {
        let status = Status::try_from(plant)?;
        Ok(PlantDetails {
            name: plant.name.clone(),
            species_link: plant.species.clone().map(|sp| (&sp, "../species").into()),
            status,
            gallery: PlantGallery::from((plant, "../img/plants")),
            growth: PlantGraphs::from(plant),
            activities: PlantActivities::from(plant),
        })
    }
}