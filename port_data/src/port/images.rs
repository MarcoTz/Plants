use super::Port;
use crate::errors::Error;
use chrono::NaiveDate;
use plants::plant::PlantImage;
use std::{
    fs::{copy, read_dir},
    path::PathBuf,
};

#[derive(Debug, PartialEq, Eq)]
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
        log::info!("Loading old images");
        let mut images = vec![];
        let files = read_dir(images_dir)?;
        for file_res in files {
            let file = file_res?;
            let path = file.path();
            let file_name = path.file_stem().ok_or(Error::Path("images".to_owned()))?;
            let file_name_str = file_name.to_str().ok_or(Error::Path("images".to_owned()))?;
            let parts = file_name_str.split('_');
            let last = parts
                .clone()
                .last()
                .ok_or(Error::FileName(file_name_str.to_owned()))?;
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
        log::info!("Converting Images");
        let mut new_images = vec![];
        for image in self.iter() {
            for plant in image.plants.iter() {
                let ext = image
                    .file_path
                    .extension()
                    .ok_or(Error::Path("No extension".to_owned()))?;
                let ext_str = ext
                    .to_str()
                    .ok_or(Error::FileName("No extension".to_owned()))?;
                let new_file_name = image.created.format(date_format).to_string() + "." + ext_str;
                let new_path_base = plants_dir.join(plant);
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

#[cfg(test)]
mod images_tests {
    use super::{OldImage, PlantImage, Port};
    use crate::port::test_common::{
        example_date1, example_date2, BASE_DIR, IMAGES_DIR, PLANTS_DIR_OUT,
    };
    use std::path::PathBuf;

    fn example_old1() -> OldImage {
        OldImage {
            created: example_date1(),
            file_path: PathBuf::from(BASE_DIR)
                .join(IMAGES_DIR)
                .join("Plant1_Plant2_01011970.jpg"),
            plants: vec!["Plant1".to_owned(), "Plant2".to_owned()],
        }
    }

    fn example_old2() -> OldImage {
        OldImage {
            created: example_date2(),
            file_path: PathBuf::from(BASE_DIR)
                .join(IMAGES_DIR)
                .join("Plant1_Plant3_02011970.jpg"),
            plants: vec!["Plant1".to_owned(), "Plant3".to_owned()],
        }
    }

    fn example_image1() -> PlantImage {
        PlantImage {
            created: example_date1(),
            file_name: "01011970.jpg".to_owned(),
            file_path: PathBuf::from(BASE_DIR)
                .join(PLANTS_DIR_OUT)
                .join("Plant1".to_owned()),
        }
    }
    fn example_image2() -> PlantImage {
        PlantImage {
            created: example_date1(),
            file_name: "01011970.jpg".to_owned(),
            file_path: PathBuf::from(BASE_DIR)
                .join(PLANTS_DIR_OUT)
                .join("Plant2".to_owned()),
        }
    }
    fn example_image3() -> PlantImage {
        PlantImage {
            created: example_date2(),
            file_name: "02011970.jpg".to_owned(),
            file_path: PathBuf::from(BASE_DIR)
                .join(PLANTS_DIR_OUT)
                .join("Plant1".to_owned()),
        }
    }
    fn example_image4() -> PlantImage {
        PlantImage {
            created: example_date2(),
            file_name: "02011970.jpg".to_owned(),
            file_path: PathBuf::from(BASE_DIR)
                .join(PLANTS_DIR_OUT)
                .join("Plant3".to_owned()),
        }
    }

    #[test]
    fn load_old() {
        let images_dir = PathBuf::from(BASE_DIR).join(IMAGES_DIR);
        let result = <Vec<OldImage> as Port<Vec<PlantImage>>>::load_old(&images_dir).unwrap();
        let expected = vec![example_old1(), example_old2()];
        assert_eq!(result, expected)
    }

    #[test]
    fn convert() {
        let img1 = example_image1();
        let img2 = example_image2();
        let img3 = example_image3();
        let img4 = example_image4();

        let img_file1 = img1.file_path.join(img1.file_name);
        let img_file2 = img2.file_path.join(img2.file_name);
        let img_file3 = img3.file_path.join(img3.file_name);
        let img_file4 = img4.file_path.join(img4.file_name);

        if img_file1.exists() {
            std::fs::remove_file(img_file1.clone()).unwrap();
        }
        if img_file2.exists() {
            std::fs::remove_file(img_file2.clone()).unwrap();
        }
        if img_file3.exists() {
            std::fs::remove_file(img_file3.clone()).unwrap();
        }
        if img_file4.exists() {
            std::fs::remove_file(img_file4.clone()).unwrap();
        }

        assert!(!img_file1.exists());
        assert!(!img_file2.exists());
        assert!(!img_file3.exists());
        assert!(!img_file4.exists());

        let plants_dir = PathBuf::from(BASE_DIR).join(PLANTS_DIR_OUT);
        if !plants_dir.exists() {
            std::fs::create_dir_all(plants_dir.clone()).unwrap();
        }
        assert!(plants_dir.exists());
        let plant1_dir = plants_dir.join("Plant1".to_owned());
        if !plant1_dir.exists() {
            std::fs::create_dir_all(plant1_dir.clone()).unwrap();
        }
        assert!(plant1_dir.exists());

        let plant2_dir = plants_dir.join("Plant2");
        if !plant2_dir.exists() {
            std::fs::create_dir_all(plant2_dir.clone()).unwrap();
        }
        assert!(plant2_dir.exists());

        let plant3_dir = plants_dir.join("Plant3");
        if !plant3_dir.exists() {
            std::fs::create_dir_all(plant3_dir.clone()).unwrap();
        }
        assert!(plant3_dir.exists());

        let result = vec![example_old1(), example_old2()]
            .convert(&(plants_dir, "%d%m%Y".to_owned()))
            .unwrap();
        let expected = vec![
            example_image1(),
            example_image2(),
            example_image3(),
            example_image4(),
        ];

        assert_eq!(result, expected);
        assert!(img_file1.exists());
        assert!(img_file2.exists());
        assert!(img_file3.exists());
        assert!(img_file4.exists());

        std::fs::remove_file(img_file1.clone()).unwrap();
        std::fs::remove_file(img_file2.clone()).unwrap();
        std::fs::remove_file(img_file3.clone()).unwrap();
        std::fs::remove_file(img_file4.clone()).unwrap();
        assert!(!img_file1.exists());
        assert!(!img_file2.exists());
        assert!(!img_file3.exists());
        assert!(!img_file4.exists());
    }

    #[test]
    fn save_new() {}
}
