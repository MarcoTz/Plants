use crate::{errors::Error, page::PageComponent};
use chrono::NaiveDate;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, A},
};
use plants::{named::Named, plant::Plant};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Status {
    health: i32,
    next_watering: Option<NaiveDate>,
    next_fertilizing: Option<NaiveDate>,
    last_watering: Option<NaiveDate>,
    last_fertilizing: Option<NaiveDate>,
    watering_frequency: Option<f32>,
    fertilizing_frequency: Option<f32>,
    current_height: f32,
    current_width: f32,
    growth_speed: f32,
    is_autowatered: bool,
    current_location: String,
    origin: String,
    age: i64,
    notes: String,
}

impl PageComponent for Status {
    fn render(&self, date_format: &str) -> HtmlElement {
        let render_item = |title: String, value: String| {
            Div {
                attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                content: Rc::new(vec![title.into(), HtmlElement::Br, value.into()].into()),
            }
            .into()
        };

        let render_option = |title: String, value: Option<String>| match value {
            None => "".to_owned().into(),
            Some(val) => render_item(title, val),
        };

        let status_items = vec![
            Div {
                attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                content: Rc::new(
                    vec![
                        "Health".to_owned().into(),
                        HtmlElement::Br,
                        Div {
                            attributes: vec![Attribute::Class(vec![
                                "health".to_owned(),
                                "health".to_owned() + &self.health.to_string(),
                            ])],
                            content: Rc::new(self.health.to_string().into()),
                        }
                        .into(),
                    ]
                    .into(),
                ),
            }
            .into(),
            render_option(
                "Next Watering".to_owned(),
                self.next_watering
                    .map(|wat| wat.format(date_format).to_string()),
            ),
            render_option(
                "Next Fertilizing".to_owned(),
                self.next_fertilizing
                    .map(|ft| ft.format(date_format).to_string()),
            ),
            render_option(
                "Last Watering".to_owned(),
                self.last_watering
                    .map(|wt| wt.format(date_format).to_string()),
            ),
            render_option(
                "Last Fertilizing".to_owned(),
                self.last_fertilizing
                    .map(|ft| ft.format(date_format).to_string()),
            ),
            render_option(
                "Watering Frequency".to_owned(),
                self.watering_frequency
                    .map(|frq| format!("{:.2} days/watering", frq)),
            ),
            render_option(
                "Fertilizing Frequency".to_owned(),
                self.fertilizing_frequency
                    .map(|frq| format!("{:.2} days/fertilizing", frq)),
            ),
            render_item(
                "Current Height".to_owned(),
                self.current_height.to_string() + "cm",
            ),
            render_item(
                "Current Width".to_owned(),
                self.current_width.to_string() + "cm",
            ),
            render_item(
                "Growth Speed".to_owned(),
                format!("{:.2} cm/day", self.growth_speed),
            ),
            render_item(
                "Is Autowatered".to_owned(),
                if self.is_autowatered {
                    "✅".to_owned()
                } else {
                    "❌".to_owned()
                },
            ),
            Div {
                attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                content: Rc::new(
                    vec![
                        "Location".to_owned().into(),
                        HtmlElement::Br,
                        A {
                            attributes: vec![Attribute::Href(
                                "../plant_overview.html#".to_owned() + &self.current_location,
                            )],
                            content: Rc::new(self.current_location.clone().into()),
                        }
                        .into(),
                    ]
                    .into(),
                ),
            }
            .into(),
            render_item("Origin".to_owned(), self.origin.clone()),
            render_item("Age".to_owned(), self.age.to_string() + " days"),
            render_item("Notes".to_owned(), self.notes.clone()),
        ];

        Div {
            attributes: vec![
                Attribute::Id("plant_status".to_owned()),
                Attribute::Class(vec![
                    "flex_container".to_owned(),
                    "alternating_children".to_owned(),
                ]),
            ],
            content: Rc::new(status_items.into()),
        }
        .into()
    }
}

impl TryFrom<&Plant> for Status {
    type Error = Error;
    fn try_from(plant: &Plant) -> Result<Status, Self::Error> {
        log::info!("loading plant status for {}", plant.info.name);
        let health = plant.get_health()?;
        let next_watering = plant.get_next_watering();
        let next_fertilizing = plant.get_next_fertilizing();
        let last_watering = plant.get_last_watering().map(|log| log.date);
        let last_fertilizing = plant.get_last_fertilizing().map(|log| log.date);
        let watering_frequency = plant.get_watering_frequency();
        let fertilizing_frequency = plant.get_fertilizing_frequency();
        //if we can get height, width and speed can never fail
        let current_height = plant.get_height()?;
        let current_width = plant.get_width().unwrap();
        let growth_speed = plant.get_growth_speed().unwrap();
        let age = plant.get_age_days();

        Ok(Status {
            health,
            next_watering,
            next_fertilizing,
            last_watering,
            last_fertilizing,
            watering_frequency,
            fertilizing_frequency,
            current_height,
            current_width,
            growth_speed,
            is_autowatered: plant.info.auto_water,
            current_location: plant.info.location.get_name(),
            origin: plant.info.origin.clone(),
            age,
            notes: plant.info.notes.join(", ").clone(),
        })
    }
}

#[cfg(test)]
mod status_tests {
    use super::{PageComponent, Status};
    use crate::test_common::{
        example_plant3, sample_date1, sample_date2, sample_date3, DATE_FORMAT,
    };
    use chrono::Local;
    use html::{
        attribute::Attribute,
        elements::{Div, HtmlElement, A},
    };
    use std::rc::Rc;

    fn example_status() -> Status {
        Status {
            health: 3,
            next_watering: Some(Local::now().date_naive()),
            next_fertilizing: Some(Local::now().date_naive()),
            last_watering: Some(sample_date1()),
            last_fertilizing: Some(sample_date2()),
            watering_frequency: Some(0.0),
            fertilizing_frequency: Some(0.0),
            current_height: 34.2,
            current_width: 83.4,
            growth_speed: 54.15,
            is_autowatered: false,
            current_location: "test location".to_owned(),
            origin: "test origin".to_owned(),
            age: (Local::now().date_naive() - sample_date3()).num_days(),
            notes: "".to_owned(),
        }
    }

    #[test]
    fn render_status() {
        let result = example_status().render(DATE_FORMAT);
        let expected = Div {
            attributes: vec![
                Attribute::Id("plant_status".to_owned()),
                Attribute::Class(vec![
                    "flex_container".to_owned(),
                    "alternating_children".to_owned(),
                ]),
            ],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Health".to_owned().into(),
                                HtmlElement::Br,
                                Div {
                                    attributes: vec![Attribute::Class(vec![
                                        "health".to_owned(),
                                        "health3".to_owned(),
                                    ])],
                                    content: Rc::new("3".to_owned().into()),
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Next Watering".to_owned().into(),
                                HtmlElement::Br,
                                Local::now()
                                    .date_naive()
                                    .format(DATE_FORMAT)
                                    .to_string()
                                    .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Next Fertilizing".to_owned().into(),
                                HtmlElement::Br,
                                Local::now()
                                    .date_naive()
                                    .format(DATE_FORMAT)
                                    .to_string()
                                    .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Last Watering".to_owned().into(),
                                HtmlElement::Br,
                                sample_date1().format(DATE_FORMAT).to_string().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Last Fertilizing".to_owned().into(),
                                HtmlElement::Br,
                                sample_date2().format(DATE_FORMAT).to_string().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Watering Frequency".to_owned().into(),
                                HtmlElement::Br,
                                "0.00 days/watering".to_owned().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Fertilizing Frequency".to_owned().into(),
                                HtmlElement::Br,
                                "0.00 days/fertilizing".to_owned().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Current Height".to_owned().into(),
                                HtmlElement::Br,
                                "34.2cm".to_owned().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Current Width".to_owned().into(),
                                HtmlElement::Br,
                                "83.4cm".to_owned().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Growth Speed".to_owned().into(),
                                HtmlElement::Br,
                                "54.15 cm/day".to_owned().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Is Autowatered".to_owned().into(),
                                HtmlElement::Br,
                                "❌".to_owned().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Location".to_owned().into(),
                                HtmlElement::Br,
                                A {
                                    attributes: vec![Attribute::Href(
                                        "../plant_overview.html#test location".to_owned(),
                                    )],
                                    content: Rc::new("test location".to_owned().into()),
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Origin".to_owned().into(),
                                HtmlElement::Br,
                                "test origin".to_owned().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Age".to_owned().into(),
                                HtmlElement::Br,
                                ((Local::now().date_naive() - sample_date3())
                                    .num_days()
                                    .to_string()
                                    + " days")
                                    .into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Class(vec!["status_item".to_owned()])],
                        content: Rc::new(
                            vec![
                                "Notes".to_owned().into(),
                                HtmlElement::Br,
                                "".to_owned().into(),
                            ]
                            .into(),
                        ),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn status_into() {
        let result = Status::try_from(&example_plant3()).unwrap();
        let expected = example_status();
        assert_eq!(result, expected)
    }

    #[test]
    fn status_into_fail() {
        let mut plant = example_plant3();
        plant.growth = vec![];
        let result = Status::try_from(&plant);
        assert!(result.is_err())
    }
}
