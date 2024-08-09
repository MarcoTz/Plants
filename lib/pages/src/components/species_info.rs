use super::page_component::PageComponent;
use html::{
    a::A,
    attribute::Attribute,
    div::Div,
    html_element::HtmlElement,
    render::Render,
    table::{Table, Td, Tr},
};
use plants::{plant::Plant, species::Species};
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
    watering_days: Option<SpeciesInfoItem>,
    fertilizing_notes: SpeciesInfoItem,
    fertilizing_days: Option<SpeciesInfoItem>,
    pruning_notes: SpeciesInfoItem,
    companions: SpeciesInfoItem,
    notes: SpeciesInfoItem,
    species_plants: SpeciesInfoItem,
}

impl PageComponent for SpeciesInfo {
    fn render(&self, _: &str) -> HtmlElement {
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
        rows.extend(vec![self.ph_range.render(), self.watering_notes.render()]);
        match &self.watering_days {
            None => (),
            Some(days) => rows.push(days.render()),
        };
        rows.push(self.fertilizing_notes.render());
        match &self.fertilizing_days {
            None => (),
            Some(days) => rows.push(days.render()),
        }
        rows.extend(vec![
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
impl From<(&str, &str)> for SpeciesInfoItem {
    fn from((name, value): (&str, &str)) -> SpeciesInfoItem {
        SpeciesInfoItem {
            name: name.to_owned(),
            value: value.to_owned(),
        }
    }
}
impl From<(&Species, &[Plant])> for SpeciesInfo {
    fn from((species, species_plants): (&Species, &[Plant])) -> SpeciesInfo {
        let temp_range_str = format!("{}-{}", species.temp_min, species.temp_max);
        let opt_temp_range_str = format!("{}-{}", species.opt_temp_min, species.opt_temp_max);
        let ph_range_str = format!("{}-{}", species.ph_min, species.ph_max);
        let mut plant_strs: Vec<String> = vec![];
        for plant in species_plants.iter() {
            plant_strs.push(
                <A as Into<HtmlElement>>::into(A {
                    attributes: vec![Attribute::Href(plant.get_url("../plants/"))],
                    content: Rc::new(plant.name.to_owned().into()),
                })
                .render(),
            )
        }
        SpeciesInfo {
            scientific_name: ("Scientific Name", species.scientific_name.as_str()).into(),
            genus: ("Genus", species.genus.as_str()).into(),
            family: ("Family", species.family.as_str()).into(),
            sunlight: (
                "Sunglight Requirements",
                species.sunlight.to_string().as_str(),
            )
                .into(),
            temp_range: ("Temperature Range", temp_range_str.as_str()).into(),
            opt_temp_range: ("Optimal Temperature Range", opt_temp_range_str.as_str()).into(),
            plant_distance: species
                .planting_distance
                .map(|dist| ("Planting Distance", dist.to_string().as_str()).into()),
            ph_range: ("pH Range", ph_range_str.as_str()).into(),
            watering_notes: ("Watering Notes", species.watering_notes.join(", ").as_str()).into(),
            watering_days: species
                .avg_watering_days
                .map(|days| ("Average Watering Days", days.to_string().as_str()).into()),
            fertilizing_notes: (
                "Fertilizing Notes",
                species.fertilizing_notes.join(", ").as_str(),
            )
                .into(),
            fertilizing_days: species
                .avg_fertilizing_days
                .map(|days| ("Average Fertilizing Days", days.to_string().as_str()).into()),
            pruning_notes: ("Pruning Notes", species.pruning_notes.join(", ").as_str()).into(),
            companions: ("Companions", species.companions.join(", ").as_str()).into(),
            notes: ("Notes", species.additional_notes.join(", ").as_str()).into(),
            species_plants: ("Plants of Species", plant_strs.join(", ").as_str()).into(),
        }
    }
}
