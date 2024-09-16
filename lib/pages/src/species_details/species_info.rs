use crate::page::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, Table, Td, Tr, A},
    render::Render,
};
use plants::{plant::Plant, species::Species};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct SpeciesInfoItem {
    name: String,
    value: String,
}

#[derive(Debug, PartialEq, Eq)]
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
    fn render(&self, date_format: &str) -> HtmlElement {
        let mut rows = vec![
            self.scientific_name.render(date_format),
            self.genus.render(date_format),
            self.family.render(date_format),
            self.sunlight.render(date_format),
            self.temp_range.render(date_format),
            self.opt_temp_range.render(date_format),
        ];
        match &self.plant_distance {
            None => (),
            Some(dist) => rows.push(dist.render(date_format)),
        };
        rows.extend(vec![
            self.ph_range.render(date_format),
            self.watering_notes.render(date_format),
        ]);
        match &self.watering_days {
            None => (),
            Some(days) => rows.push(days.render(date_format)),
        };
        rows.push(self.fertilizing_notes.render(date_format));
        match &self.fertilizing_days {
            None => (),
            Some(days) => rows.push(days.render(date_format)),
        }
        rows.extend(vec![
            self.pruning_notes.render(date_format),
            self.companions.render(date_format),
            self.notes.render(date_format),
            self.species_plants.render(date_format),
        ]);
        Div {
            attributes: vec![Attribute::Id("species_details_container".to_owned())],
            content: Rc::new(
                Table {
                    attributes: vec![],
                    rows,
                }
                .into(),
            ),
        }
        .into()
    }
}

impl PageComponent for SpeciesInfoItem {
    fn render(&self, _: &str) -> HtmlElement {
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
        .into()
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
        log::info!("Loading species info for {}", species.name);
        let temp_range_str = format!("{}-{}", species.temp_min, species.temp_max);
        let opt_temp_range_str = format!("{}-{}", species.opt_temp_min, species.opt_temp_max);
        let ph_range_str = format!("{}-{}", species.ph_min, species.ph_max);
        let mut plant_strs: Vec<String> = vec![];
        for plant in species_plants.iter() {
            plant_strs.push(
                <A as Into<HtmlElement>>::into(A {
                    attributes: vec![Attribute::Href(plant.get_url("../plants/"))],
                    content: Rc::new(plant.info.name.to_owned().into()),
                })
                .render(),
            )
        }
        SpeciesInfo {
            scientific_name: ("Scientific Name", species.scientific_name.as_str()).into(),
            genus: ("Genus", species.genus.as_str()).into(),
            family: ("Family", species.family.as_str()).into(),
            sunlight: (
                "Sunlight Requirements",
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

#[cfg(test)]
mod species_info_tests {
    use super::{PageComponent, SpeciesInfo, SpeciesInfoItem};
    use crate::test_common::{
        example_plant1, example_plant2, example_plant3, example_species, DATE_FORMAT,
    };
    use html::elements::{Div, Td, Tr};
    use html::{
        attribute::Attribute,
        elements::{Table, A},
        render::Render,
    };
    use std::rc::Rc;

    fn example_info() -> SpeciesInfo {
        let links = vec![
            A {
                attributes: vec![Attribute::Href(example_plant1().get_url("../plants/"))],
                content: Rc::new("Plant1".to_owned().into()),
            }
            .render(),
            A {
                attributes: vec![Attribute::Href(example_plant2().get_url("../plants/"))],
                content: Rc::new("Plant2".to_owned().into()),
            }
            .render(),
            A {
                attributes: vec![Attribute::Href(example_plant3().get_url("../plants/"))],
                content: Rc::new("Plant3".to_owned().into()),
            }
            .render(),
        ];
        SpeciesInfo {
            scientific_name: ("Scientific Name", "dummy").into(),
            genus: ("Genus", "dummy").into(),
            family: ("Family", "dummy").into(),
            sunlight: ("Sunlight Requirements", "Direct").into(),
            temp_range: ("Temperature Range", "0-100").into(),
            opt_temp_range: ("Optimal Temperature Range", "0-100").into(),
            plant_distance: None,
            ph_range: ("pH Range", "0-10").into(),
            watering_notes: ("Watering Notes", "").into(),
            watering_days: Some(("Average Watering Days", "1").into()),
            fertilizing_notes: ("Fertilizing Notes", "").into(),
            fertilizing_days: Some(("Average Fertilizing Days", "1").into()),
            pruning_notes: ("Pruning Notes", "").into(),
            companions: ("Companions", "").into(),
            notes: ("Notes", "").into(),
            species_plants: ("Plants of Species", links.join(", ").as_str()).into(),
        }
    }

    #[test]
    fn render_info() {
        let info = example_info();
        let result = info.render(DATE_FORMAT);
        let expected = Div {
            attributes: vec![Attribute::Id("species_details_container".to_owned())],
            content: Rc::new(
                Table {
                    attributes: vec![],
                    rows: vec![
                        info.scientific_name.render(DATE_FORMAT),
                        info.genus.render(DATE_FORMAT),
                        info.family.render(DATE_FORMAT),
                        info.sunlight.render(DATE_FORMAT),
                        info.temp_range.render(DATE_FORMAT),
                        info.opt_temp_range.render(DATE_FORMAT),
                        info.ph_range.render(DATE_FORMAT),
                        info.watering_notes.render(DATE_FORMAT),
                        info.watering_days.unwrap().render(DATE_FORMAT),
                        info.fertilizing_notes.render(DATE_FORMAT),
                        info.fertilizing_days.unwrap().render(DATE_FORMAT),
                        info.pruning_notes.render(DATE_FORMAT),
                        info.companions.render(DATE_FORMAT),
                        info.notes.render(DATE_FORMAT),
                        info.species_plants.render(DATE_FORMAT),
                    ],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_item() {
        let result = SpeciesInfoItem {
            name: "Name".to_owned(),
            value: "test species".to_owned(),
        }
        .render(DATE_FORMAT);
        let expected = Tr {
            attributes: vec![],
            cols: vec![
                Td {
                    content: Rc::new("Name".to_owned().into()),
                },
                Td {
                    content: Rc::new("test species".to_owned().into()),
                },
            ],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn info_into() {
        let result = SpeciesInfo::from((
            &example_species(),
            vec![example_plant1(), example_plant2(), example_plant3()].as_slice(),
        ));
        let expected = example_info();
        assert_eq!(result, expected)
    }

    #[test]
    fn item_into() {
        let result = SpeciesInfoItem::from(("Name", "test species"));
        let expected = SpeciesInfoItem {
            name: "Name".to_owned(),
            value: "test species".to_owned(),
        };
        assert_eq!(result, expected)
    }
}
