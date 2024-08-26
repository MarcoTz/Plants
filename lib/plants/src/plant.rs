use super::{
    errors::Error,
    growth_item::GrowthItem,
    log_item::LogItem,
    serialize::{date_serializer, species_serializer},
    species::Species,
};
use chrono::{Local, NaiveDate, TimeDelta};
use serde::{Deserialize, Serialize};
use std::{cmp::max, path::PathBuf};

#[derive(Clone, Debug)]
pub struct PlantImage {
    pub created: NaiveDate,
    pub file_name: String,
    pub file_path: PathBuf,
}

#[derive(Clone, Debug)]
pub enum PlantSpecies {
    Species(Box<Species>),
    Other(String),
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct PlantInfo {
    pub name: String,
    #[serde(with = "species_serializer")]
    pub species: PlantSpecies,
    pub location: String,
    pub origin: String,
    #[serde(with = "date_serializer")]
    pub obtained: NaiveDate,
    pub auto_water: bool,
    pub notes: Vec<String>,
}
#[derive(Clone, Debug)]
pub struct Plant {
    pub info: PlantInfo,
    pub growth: Vec<GrowthItem>,
    pub activities: Vec<LogItem>,
    pub images: Vec<PlantImage>,
}

impl Plant {
    fn get_activities(&self, activity_name: &str) -> Vec<LogItem> {
        log::info!("Getting activitiyes {activity_name} for {}", self.info.name);
        let mut activities = vec![];
        for activity in self.activities.iter() {
            if activity.activity.to_lowercase().trim() == activity_name.to_lowercase().trim() {
                activities.push(activity.clone())
            }
        }
        if activities.is_empty() {
            log::warn!("No activities {activity_name} for {}", self.info.name)
        }
        activities
    }

    fn get_watering_activities(&self) -> Vec<LogItem> {
        self.get_activities("watering")
    }

    pub fn get_last_watering(&self) -> Option<LogItem> {
        log::info!("Getting last watering activity for {}", self.info.name);
        let mut watering_activities = self.get_watering_activities();
        watering_activities.sort_by(|log1, log2| log1.date.cmp(&log2.date));
        watering_activities.last().cloned()
    }

    pub fn get_last_fertilizing(&self) -> Option<LogItem> {
        log::info!("Geting last fertilizing activity for {}", self.info.name);
        let mut watering_activities = self.get_fertilizing_activities();
        watering_activities.sort_by(|log1, log2| log1.date.cmp(&log2.date));
        watering_activities.last().cloned()
    }

    fn get_fertilizing_activities(&self) -> Vec<LogItem> {
        self.get_activities("fertilizing")
    }

    pub fn get_age_days(&self) -> i64 {
        log::info!("Getting age for {}", self.info.name);
        let today = Local::now().date_naive();
        let time_diff = today - self.info.obtained;
        time_diff.num_days()
    }

    fn get_next_activity_date(&self, activity_name: &str) -> Option<NaiveDate> {
        log::info!(
            "Getting next activity {activity_name} for {}",
            self.info.name
        );
        let self_activities = self.get_activities(activity_name);
        let m_last_activity = self_activities.iter().max();
        match (m_last_activity, &self.info.species) {
            (None, _) => Some(Local::now().date_naive()),
            (_, PlantSpecies::Other(_)) => None,
            (Some(last_activity), PlantSpecies::Species(species)) => {
                let activity_delta = species.get_activity_delta(activity_name)?;
                Some(max(
                    last_activity.date + activity_delta,
                    Local::now().date_naive(),
                ))
            }
        }
    }

    pub fn get_next_watering(&self) -> Option<NaiveDate> {
        self.get_next_activity_date("watering")
    }

    pub fn get_next_fertilizing(&self) -> Option<NaiveDate> {
        self.get_next_activity_date("fertilizing")
    }

    fn get_last_growth(&self) -> Result<GrowthItem, Error> {
        log::info!("Getting next growth for plant {}", self.info.name);

        let last_growth = self
            .growth
            .iter()
            .max()
            .ok_or(Error::GrowthError(self.info.name.clone()))?;
        Ok(last_growth.clone())
    }
    pub fn get_height(&self) -> Result<f32, Error> {
        log::info!("Getting height for plant {}", self.info.name);
        let last_growth = self.get_last_growth()?;
        Ok(last_growth.height_cm)
    }

    pub fn get_width(&self) -> Result<f32, Error> {
        log::info!("Getting width for plant {}", self.info.name);
        let last_growth = self.get_last_growth()?;
        Ok(last_growth.width_cm)
    }

    pub fn get_health(&self) -> Result<i32, Error> {
        log::info!("Getting Health for plant {}", self.info.name);
        let last_growth = self.get_last_growth()?;
        Ok(last_growth.health)
    }

    fn get_activity_frequency(&self, activity_name: &str) -> Option<f32> {
        log::info!(
            "Getting frequency of activity {} for {}",
            activity_name,
            self.info.name
        );
        let self_activities = self.get_activities(activity_name);
        let first_activity = self_activities.iter().min()?;
        let last_activity = self_activities.iter().max()?;
        let activity_diff = last_activity.date - first_activity.date;
        match self_activities.len() {
            0 => None,
            n => Some(activity_diff.num_days() as f32 / n as f32),
        }
    }

    pub fn get_fertilizing_frequency(&self) -> Option<f32> {
        self.get_activity_frequency("fertilizing")
    }

    pub fn get_watering_frequency(&self) -> Option<f32> {
        self.get_activity_frequency("watering")
    }

    pub fn get_growth_speed(&self) -> Result<f32, Error> {
        log::info!("Getting growth seped for {}", self.info.name);
        let self_growth = self
            .growth
            .iter()
            .filter(|it| it.plant == self.info.name)
            .cloned();
        let first_growth = self_growth
            .clone()
            .min()
            .ok_or(Error::GrowthError(self.info.name.clone()))?;
        let last_growth = self_growth
            .max()
            .ok_or(Error::GrowthError(self.info.name.clone()))?;
        let height_diff = last_growth.height_cm - first_growth.height_cm;
        let width_diff = last_growth.width_cm - first_growth.width_cm;
        let time_diff = (last_growth.date - first_growth.date).num_days() as f32;
        if time_diff == 0.0 {
            Ok(0.0)
        } else {
            let height_speed = height_diff / time_diff;
            let width_speed = width_diff / time_diff;
            let avg = (height_speed + width_speed) / 2.0;
            Ok(avg)
        }
    }

    pub fn get_url(&self, base: &str) -> String {
        let mut url = base.to_owned();
        url.push('/');
        url.push_str(&self.info.name.replace(' ', ""));
        url.push_str(".html");
        url
    }

    pub fn get_next_growth(&self) -> NaiveDate {
        log::info!("Getting next growth for {}", self.info.name);
        match self.get_last_growth() {
            Err(_) => Local::now().date_naive(),
            Ok(last_growth) => max(
                last_growth.date + TimeDelta::days(14),
                Local::now().date_naive(),
            ),
        }
    }

    pub fn get_preview_image_url(&self, base: &str) -> Option<String> {
        let image = self.images.first().cloned()?;
        let image_url = base.to_owned() + &image.file_name;
        Some(image_url)
    }
}

#[derive(Debug)]
enum CmpOptions {
    Height,
    Width,
    GrowthSpeed,
    Age,
}

fn sort_plants(plants: &[Plant], cmp: CmpOptions) -> Result<Vec<(f32, &Plant)>, Error> {
    let cmp_fun = match cmp {
        CmpOptions::Height => |p: &Plant| p.get_height(),
        CmpOptions::Width => |p: &Plant| p.get_width(),
        CmpOptions::GrowthSpeed => |p: &Plant| p.get_growth_speed(),
        CmpOptions::Age => |p: &Plant| Ok(p.get_age_days() as f32),
    };

    let mut plants_with_vals: Vec<(f32, &Plant)> = plants
        .iter()
        .map(|p| cmp_fun(p).map(|val| (val, p)))
        .collect::<Result<Vec<(f32, &Plant)>, Error>>()?;
    plants_with_vals.sort_by(|(val1, _), (val2, _)| val1.partial_cmp(val2).unwrap());
    Ok(plants_with_vals)
}

pub fn sort_height(plants: &[Plant]) -> Result<Vec<(f32, &Plant)>, Error> {
    sort_plants(plants, CmpOptions::Height)
}

pub fn sort_width(plants: &[Plant]) -> Result<Vec<(f32, &Plant)>, Error> {
    sort_plants(plants, CmpOptions::Width)
}

pub fn sort_speed(plants: &[Plant]) -> Result<Vec<(f32, &Plant)>, Error> {
    sort_plants(plants, CmpOptions::GrowthSpeed)
}

pub fn sort_age(plants: &[Plant]) -> Result<Vec<(f32, &Plant)>, Error> {
    sort_plants(plants, CmpOptions::Age)
}
