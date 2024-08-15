use crate::page::PageComponent;
use chrono::NaiveDate;
use html::{
    attribute::Attribute,
    elements::{Canvas, Div, HeaderSize, Headline, HtmlElement, Script},
};
use plants::plant::Plant;
use std::rc::Rc;

pub struct GraphValues<T> {
    name: String,
    values: Vec<T>,
}
pub struct PlantGraph<T> {
    title: String,
    id: String,
    x_values: GraphValues<NaiveDate>,
    y_values: Vec<GraphValues<T>>,
}
pub struct PlantGraphs {
    growth_graph: PlantGraph<f32>,
    health_graph: PlantGraph<i32>,
}

impl PageComponent for PlantGraphs {
    fn render(&self, date_format: &str) -> HtmlElement {
        Div {
            attributes: vec![
                Attribute::Id("plant_growth_log_container".to_owned()),
                Attribute::Class(vec!["flex_container".to_owned()]),
            ],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H2,
                        content: Rc::new("Growth Log".to_owned().into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_graph".to_owned()])],
                        content: Rc::new(self.growth_graph.render(date_format)),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["plant_graph".to_owned()])],
                        content: Rc::new(self.health_graph.render(date_format)),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}

impl<T: ToString> PageComponent for PlantGraph<T> {
    fn render(&self, date_format: &str) -> HtmlElement {
        let x_values_str = self
            .x_values
            .values
            .iter()
            .map(|val| format!("\"{}\"", val.format(date_format)))
            .collect::<Vec<String>>()
            .join(", ");

        let mut y_strs = "".to_owned();
        for y_value in self.y_values.iter() {
            y_strs.push_str(&format!("{} = [", y_value.name));
            y_strs.push_str(
                &y_value
                    .values
                    .iter()
                    .map(|val| val.to_string())
                    .collect::<Vec<String>>()
                    .join(" ,"),
            );
            y_strs.push_str("]; ");
        }

        vec![
            Headline {
                attributes: vec![],
                size: HeaderSize::H3,
                content: Rc::new(self.title.clone().into()),
            }
            .into(),
            Canvas {
                attributes: vec![Attribute::Id(self.id.clone())],
            }
            .into(),
            Script {
                attributes: vec![],
                content: format!(
                    "{}=[{}]; {}",
                    self.x_values.name.clone(),
                    x_values_str,
                    y_strs
                ),
            }
            .into(),
        ]
        .into()
    }
}

impl From<&Plant> for PlantGraphs {
    fn from(plant: &Plant) -> PlantGraphs {
        let growth_dates: Vec<NaiveDate> = plant.growth.iter().map(|growth| growth.date).collect();
        let growth_heights = plant.growth.iter().map(|growth| growth.height_cm).collect();
        let growth_widths = plant.growth.iter().map(|growth| growth.width_cm).collect();
        let growth_healths = plant.growth.iter().map(|growth| growth.health).collect();
        PlantGraphs {
            growth_graph: PlantGraph {
                title: "Plant Growth".to_owned(),
                id: "growth_chart".to_owned(),
                x_values: GraphValues {
                    name: "growth_dates".to_owned(),
                    values: growth_dates.clone(),
                },
                y_values: vec![
                    GraphValues {
                        name: "growth_heights".to_owned(),
                        values: growth_heights,
                    },
                    GraphValues {
                        name: "growth_widths".to_owned(),
                        values: growth_widths,
                    },
                ],
            },
            health_graph: PlantGraph {
                title: "Plant Health".to_owned(),
                id: "health_chart".to_owned(),
                x_values: GraphValues {
                    name: "health_dates".to_owned(),
                    values: growth_dates,
                },
                y_values: vec![GraphValues {
                    name: "health_healths".to_owned(),
                    values: growth_healths,
                }],
            },
        }
    }
}
