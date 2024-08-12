use super::{
    components::{page_component::PageComponent, plant_activity_table::PlantActivityTable},
    page::Page,
    shared::html_head::HtmlHead,
};

use html::html_element::HtmlElement;
use plants::plant::Plant;

pub struct Activities {
    pub activity_table: PlantActivityTable,
}

impl Page for Activities {
    fn get_content(&self, date_format: &str) -> HtmlElement {
        self.activity_table.render(date_format)
    }

    fn get_head(&self) -> HtmlHead {
        let styles = vec![
            "css/header.css".to_owned(),
            "css/footer.css".to_owned(),
            "css/main.css".to_owned(),
        ];
        HtmlHead {
            title: "Activities".to_owned(),
            styles,
        }
    }
}

impl From<&[Plant]> for Activities {
    fn from(plants: &[Plant]) -> Activities {
        Activities {
            activity_table: PlantActivityTable::from((plants, true, true)),
        }
    }
}
