use super::{
    components::{page_component::PageComponent, plant_contents::PlantContents},
    errors::Error,
    page::Page,
    shared::html_head::HtmlHead,
};
use html::{
    headline::{HeaderSize, Headline},
    html_element::HtmlElement,
};
use plants::plant::Plant;
use std::rc::Rc;

pub struct PlantDetails {
    pub plant_name: String,
    pub plant_species: Option<String>,
    pub plant_content: PlantContents,
}

impl Page for PlantDetails {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        let plant_header = Headline {
            attributes: vec![],
            size: HeaderSize::H1,
            content: {
                let plant_name_str = self.plant_name.clone();
                let plant_species_str = self.plant_species.clone().unwrap_or("".to_owned());
                Rc::new(format!("{plant_name_str} {plant_species_str}").into())
            },
        }
        .into();
        vec![plant_header, self.plant_content.render(date_format)].into()
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "../css/main.css".to_owned(),
            "../css/header.css".to_owned(),
            "../css/footer.css".to_owned(),
        ];
        HtmlHead {
            title: self.plant_name.clone(),
            styles,
        }
    }
}
impl TryFrom<&Plant> for PlantDetails {
    type Error = Error;
    fn try_from(plant: &Plant) -> Result<PlantDetails, Error> {
        let plant_species = plant.species.clone().map(|sp| sp.name.clone());
        let img_base = "../img/plants";
        let plant_content = PlantContents::try_from((plant, img_base))?;
        Ok(PlantDetails {
            plant_name: plant.name.clone(),
            plant_species,
            plant_content,
        })
    }
}
