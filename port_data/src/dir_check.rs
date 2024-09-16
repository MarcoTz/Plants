use super::errors::Error;
use std::{
    fs::create_dir,
    path::{Path, PathBuf},
};

const DATA_DIR_OLD: &str = "data_old";
const DATA_DIR_NEW: &str = "data";
const PLANTS_DIR: &str = "Plants";
const SPECIES_DIR_OLD: &str = "PlantSpecies";
const SPECIES_DIR_NEW: &str = "Species";
const LOGS_DIR: &str = "Logs";

#[derive(Debug, PartialEq, Eq)]
pub struct Directories {
    pub data_dir_in: PathBuf,
    pub data_dir_out: PathBuf,
    pub plants_dir_in: PathBuf,
    pub plants_dir_out: PathBuf,
    pub species_dir_in: PathBuf,
    pub species_dir_out: PathBuf,
    pub logs_dir_in: PathBuf,
    pub logs_dir_out: PathBuf,
}

impl Default for Directories {
    fn default() -> Directories {
        let data_dir_in = PathBuf::from(DATA_DIR_OLD);
        let data_dir_out = PathBuf::from(DATA_DIR_NEW);
        Directories {
            plants_dir_in: data_dir_in.clone().join(PLANTS_DIR),
            plants_dir_out: data_dir_out.clone().join(PLANTS_DIR),
            species_dir_in: data_dir_in.clone().join(SPECIES_DIR_OLD),
            species_dir_out: data_dir_out.clone().join(SPECIES_DIR_NEW),
            logs_dir_in: data_dir_in.clone().join(LOGS_DIR),
            logs_dir_out: data_dir_out.clone().join(LOGS_DIR),
            data_dir_in,
            data_dir_out,
        }
    }
}

impl Directories {
    pub fn ensure_exists(&self) -> Result<(), Error> {
        self.ensure_in_exists(&self.data_dir_in)?;
        self.ensure_in_exists(&self.plants_dir_in)?;
        self.ensure_in_exists(&self.species_dir_in)?;
        self.ensure_in_exists(&self.logs_dir_in)?;

        self.ensure_out_exists(&self.data_dir_out)?;
        self.ensure_out_exists(&self.plants_dir_out)?;
        self.ensure_out_exists(&self.species_dir_out)?;
        self.ensure_out_exists(&self.logs_dir_out)?;

        Ok(())
    }

    fn ensure_in_exists(&self, dir: &Path) -> Result<(), Error> {
        if !dir.exists() {
            Err(Error::MissingFiles(dir.to_path_buf()))
        } else {
            Ok(())
        }
    }

    fn ensure_out_exists(&self, dir: &Path) -> Result<(), Error> {
        if !dir.exists() {
            create_dir(dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod dir_check_tests {
    use super::{
        Directories, DATA_DIR_NEW, DATA_DIR_OLD, LOGS_DIR, PLANTS_DIR, SPECIES_DIR_NEW,
        SPECIES_DIR_OLD,
    };
    use std::path::{Path, PathBuf};

    #[test]
    fn default_dirs() {
        let result = Directories::default();
        let expected = Directories {
            plants_dir_in: PathBuf::from(DATA_DIR_OLD).join(PLANTS_DIR),
            plants_dir_out: PathBuf::from(DATA_DIR_NEW).join(PLANTS_DIR),
            species_dir_in: PathBuf::from(DATA_DIR_OLD).join(SPECIES_DIR_OLD),
            species_dir_out: PathBuf::from(DATA_DIR_NEW).join(SPECIES_DIR_NEW),
            logs_dir_in: PathBuf::from(DATA_DIR_OLD).join(LOGS_DIR),
            logs_dir_out: PathBuf::from(DATA_DIR_NEW).join(LOGS_DIR),
            data_dir_in: PathBuf::from(DATA_DIR_OLD),
            data_dir_out: PathBuf::from(DATA_DIR_NEW),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn ensure_exists() {
        let dirs = Directories::default();
        std::env::set_current_dir(Path::new("../")).unwrap();
        dirs.ensure_exists().unwrap();
        assert!(dirs.data_dir_in.exists());
        assert!(dirs.data_dir_out.exists());
        assert!(dirs.plants_dir_in.exists());
        assert!(dirs.plants_dir_out.exists());
        assert!(dirs.species_dir_in.exists());
        assert!(dirs.logs_dir_in.exists());
        assert!(dirs.logs_dir_out.exists());
    }
}
