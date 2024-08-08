use super::{
    super::errors::Error, page_component::PageComponent, plant_activities::PlantActivities,
    plant_gallery::PlantGallery, plant_growth::PlantGrowth, plant_status::PlantStatus,
};
use html::{attribute::Attribute, div::Div, html_element::HtmlElement};
use plants::plant::Plant;
use std::rc::Rc;

pub struct PlantContents {
    gallery: PlantGallery,
    status: PlantStatus,
    growth: PlantGrowth,
    activities: PlantActivities,
}

impl PageComponent for PlantContents {
    fn render(&self, date_format: &str) -> HtmlElement {
        let details_content = vec![
            self.gallery.render(date_format),
            Div {
                attributes: vec![Attribute::Id("plant_info".to_owned())],
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
        ];
        Div {
            attributes: vec![Attribute::Id("plant_content".to_owned())],
            content: Rc::new(details_content.into()),
        }
        .into()
    }
}

impl TryFrom<&Plant> for PlantContents {
    type Error = Error;
    fn try_from(plant: &Plant) -> Result<PlantContents, Error> {
        let status = PlantStatus::try_from(plant)?;
        Ok(PlantContents {
            gallery: PlantGallery::from(plant),
            status,
            growth: PlantGrowth::from(()),
            activities: PlantActivities::from(plant),
        })
    }
}
