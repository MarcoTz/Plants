use super::Port;
use crate::errors::Error;
use chrono::NaiveDate;
use plants::plant::PlantImage;
use std::{
    fs::{copy, read_dir},
    path::PathBuf,
};

pub struct OldImage {
    created: NaiveDate,
    file_path: PathBuf,
    plants: Vec<String>,
}
impl Port<Vec<PlantImage>> for Vec<OldImage> {
    type LoadArgs = PathBuf;
    type SaveArgs = PathBuf;
    type ConvertArgs = (PathBuf, String);

    fn load_old(images_dir: &Self::LoadArgs) -> Result<Self, Error> {
        let mut images = vec![];
        let files = read_dir(images_dir)?;
        for file_res in files {
            let file = file_res?;
            let path = file.path();
            let file_name = path
                .file_stem()
                .ok_or(Error::PathError("images".to_owned()))?;
            let file_name_str = file_name
                .to_str()
                .ok_or(Error::PathError("images".to_owned()))?;
            let parts = file_name_str.split("_");
            let last = parts
                .clone()
                .last()
                .ok_or(Error::FileNameError(file_name_str.to_owned()))?;
            let mut plants = vec![];
            let created = NaiveDate::parse_from_str(last, "%d%m%Y")?;
            for part in parts {
                if part != last {
                    plants.push(part.to_owned())
                }
            }
            let img = OldImage {
                created,
                plants,
                file_path: file.path(),
            };
            images.push(img)
        }
        Ok(images)
    }

    fn convert(
        self,
        (plants_dir, date_format): &Self::ConvertArgs,
    ) -> Result<Vec<PlantImage>, Error> {
        let mut new_images = vec![];
        for image in self.iter() {
            for plant in image.plants.iter() {
                let plant_dir_name = plant.replace(' ', "");
                let ext = image
                    .file_path
                    .extension()
                    .ok_or(Error::PathError("No extension".to_owned()))?;
                let ext_str = ext
                    .to_str()
                    .ok_or(Error::FileNameError("No extension".to_owned()))?;
                let new_file_name = image.created.format(&date_format).to_string() + "." + ext_str;
                let new_path_base = plants_dir.join(plant_dir_name);
                if !new_path_base.exists() {
                    log::warn!("Could not find {new_path_base:?}");
                    continue;
                }
                let new_path = new_path_base.join(new_file_name.clone());
                log::info!("copying {:?} to {:?}", image.file_path, new_path);
                copy(image.file_path.clone(), new_path)?;
                let new_image = PlantImage {
                    created: image.created,
                    file_name: new_file_name,
                    file_path: new_path_base,
                };
                new_images.push(new_image)
            }
        }
        Ok(new_images)
    }

    fn save_new(_: Vec<PlantImage>, _: &Self::SaveArgs) -> Result<(), Error> {
        Ok(())
    }
}

/*
 pub fn load_images(image_dir: &PathBuf, plant_name: &str) -> Result<Vec<PlantImage>, Error> {
    let mut plant_images = vec![];
    let dir_files = fs::read_dir(image_dir).map_err(|err| {
        <FSError as Into<Error>>::into(FSError {
            path: image_dir.clone(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })
    })?;
    for dir_file in dir_files {
        let dir_file = dir_file.map_err(|err| FSError {
            path: image_dir.clone(),
            err_msg: err.to_string(),
            access: AccessType::Read,
        })?;
        let path = dir_file.path();
        let file_base = path.file_name().ok_or(FSError {
            path: path.clone(),
            err_msg: "Could not find path".to_owned(),
            access: AccessType::Read,
        })?;
        let file_name = file_base.to_str().ok_or(FSError {
            path: image_dir.clone(),
            err_msg: "Could not get name as string".to_owned(),
            access: AccessType::Read,
        })?;
        if file_name.contains(plant_name) {
            let file_end = file_name.split('_').last().ok_or(FSError {
                path: path.clone(),
                err_msg: "Filename did not contain date".to_owned(),
                access: AccessType::Read,
            })?;
            let parts = file_end.split('.').collect::<Vec<&str>>();

            let date_str = parts.first().ok_or(FSError {
                path: path.clone(),
                err_msg: "Filename did not contain date".to_owned(),
                access: AccessType::Read,
            })?;
            let created = NaiveDate::parse_from_str(date_str, "%d%m%Y")?;
            let file_path = path.parent().ok_or(FSError {
                path: path.clone(),
                err_msg: "Could not get parent".to_owned(),
                access: AccessType::Read,
            })?;
            let image = PlantImage {
                created,
                file_name: file_name.to_owned(),
                file_path: file_path.to_path_buf(),
            };
            plant_images.push(image)
        }
    }
    Ok(plant_images)
}
 */
