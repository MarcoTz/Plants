use super::{
    super::{
        super::html::{attribute::Attribute, div::Div, html_element::HtmlElement},
        page::PageComponent,
    },
    footer::Footer,
    header::Header,
    plant_activities::PlantActivities,
    plant_gallery::PlantGallery,
    plant_growth::PlantGrowth,
    plant_status::PlantStatus,
};
use std::rc::Rc;

pub struct PlantContents {
    gallery: PlantGallery,
    status: PlantStatus,
    growth: PlantGrowth,
    activities: PlantActivities,
    header: Header,
    footer: Footer,
}

impl PageComponent for PlantContents {
    fn render(&self) -> HtmlElement {
        let details_content = vec![
            self.header.render(),
            self.gallery.render(),
            Div {
                attributes: vec![Attribute::Id("plant_info".to_owned())],
                content: Rc::new(vec![self.status.render(), self.growth.render()].into()),
            }
            .into(),
            self.activities.render(),
            self.footer.render(),
        ];
        Div {
            attributes: vec![Attribute::Id("plant_content".to_owned())],
            content: Rc::new(details_content.into()),
        }
        .into()
    }
}
