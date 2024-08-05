use super::super::{
    super::html::{
        attribute::Attribute,
        div::Div,
        html_element::HtmlElement,
        table::{Table, Td, Tr},
    },
    page::PageComponent,
};
use std::rc::Rc;

pub struct SpeciesInfoItem {
    name: String,
    value: String,
}
pub struct SpeciesInfo {
    scientific_name: SpeciesInfoItem,
    genus: SpeciesInfoItem,
    family: SpeciesInfoItem,
    sunlight: SpeciesInfoItem,
    temp_range: SpeciesInfoItem,
    opt_temp_range: SpeciesInfoItem,
    plant_distance: Option<SpeciesInfoItem>,
    ph_range: SpeciesInfoItem,
    watering_notes: SpeciesInfoItem,
    watering_days: SpeciesInfoItem,
    fertilizing_notes: SpeciesInfoItem,
    fertilizing_days: SpeciesInfoItem,
    pruning_notes: SpeciesInfoItem,
    companions: SpeciesInfoItem,
    notes: SpeciesInfoItem,
    species_plants: SpeciesInfoItem,
}

impl PageComponent for SpeciesInfo {
    fn render(&self) -> HtmlElement {
        let mut rows = vec![
            self.scientific_name.render(),
            self.genus.render(),
            self.family.render(),
            self.sunlight.render(),
            self.temp_range.render(),
            self.opt_temp_range.render(),
        ];
        match &self.plant_distance {
            None => (),
            Some(dist) => rows.push(dist.render()),
        };
        rows.extend(vec![
            self.ph_range.render(),
            self.watering_notes.render(),
            self.watering_days.render(),
            self.fertilizing_notes.render(),
            self.fertilizing_days.render(),
            self.pruning_notes.render(),
            self.companions.render(),
            self.notes.render(),
            self.species_plants.render(),
        ]);
        Div {
            attributes: vec![Attribute::Id("species_details_container".to_owned())],
            content: Rc::new(Table { rows }.into()),
        }
        .into()
    }
}

impl SpeciesInfoItem {
    fn render(&self) -> Tr {
        Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new(self.name.clone().into()),
                },
                Td {
                    content: Rc::new(self.value.clone().into()),
                },
            ],
        }
    }
}
